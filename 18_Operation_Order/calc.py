#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
equations = open(filename).read().strip().split("\n")


def tokenize(term):
    tokens = []
    for x in term.split(" "):
        if x in "+*":
            tokens.append(x)
        else:
            leading = 0
            trailing = 0
            for c in x:
                if c == "(":
                    leading += 1
                else:
                    break
            for c in x[::-1]:
                if c == ")":
                    trailing += 1
                else:
                    break
            tokens.extend(["("] * leading)
            tokens.append(int(x[leading:len(x) - trailing]))
            tokens.extend([")"] * trailing)
    return tokens


def paranthesize_addition(tokens):

    def find_operand_end(i, direction):
        nesting = tokens[i] in ["(", ")"]
        while nesting > 0:
            i += direction
            nesting_delta = {"(": 1, ")": -1}.get(tokens[i]) or 0
            nesting_delta *= direction
            nesting += nesting_delta
        return i

    i = 0
    while i < len(tokens):
        if tokens[i] == "+":
            l = find_operand_end(i-1, -1)
            r = find_operand_end(i+1, 1)
            tokens.insert(r + 1, ")")
            tokens.insert(l, "(")
            i += 1
        i += 1

    return tokens


def calculate(term, plus_precedence=False):

    def solve(eq):
        while "(" in eq:
            start = eq.index("(")
            end = start
            nesting = 1
            while nesting > 0:
                end += 1
                if eq[end] == "(":
                    nesting += 1
                elif eq[end] == ")":
                    nesting -= 1
            partial_sol = solve(eq[start + 1:end])
            del eq[start:end]
            eq[start] = partial_sol
        assert ")" not in eq
        # No more paranthesised terms in eq, now just calculate
        while len(eq) > 1:
            operator = eq[1]
            operand = eq[2]
            if operator == "+":
                eq[0] += operand
            elif operator == "*":
                eq[0] *= operand
            else:
                raise Exception(operator, operand)
            del eq[1:3]
        return eq[0]

    tokens = tokenize(term)

    if plus_precedence:
        tokens = paranthesize_addition(tokens)

    return solve(tokens)


print(sum(calculate(term, plus_precedence=True) for term in equations))
