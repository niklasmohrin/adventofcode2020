#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
data = open(filename).read().strip().split("\n")


def to_number(code, zero="FL", one="BR"):
    res = 0
    bit = 1
    for digit in reversed(code):
        if digit in one:
            res += bit
        elif digit not in zero:
            raise

        bit <<= 1
    return res


# Part One
# print(max(to_number(code) for code in data))

# Part Two
taken_seats = list(sorted(map(to_number, data)))
for i in range(1, len(taken_seats)):
    if taken_seats[i] != taken_seats[i-1] + 1:
        print(f"My seat is between {taken_seats[i]} and {taken_seats[i-1]}")
