#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
rows = open(filename).read().strip().split("\n")


# Part One
# def row_is_valid(row):
#     rule, pw = row.split(": ")
#     count, char = rule.split(" ")
#     minimum, maximum = map(int, count.split("-"))
#     return minimum <= pw.count(char) <= maximum

# Part Two
def row_is_valid(row):
    rule, pw = row.split(": ")
    count, char = rule.split(" ")
    pos1, pos2 = map(int, count.split("-"))
    # Convert to zero-index
    pos1 -= 1
    pos2 -= 1
    # Exactly one should be equal to the character
    return (pw[pos1] == char) ^ (pw[pos2] == char)


valid_rows = list(filter(row_is_valid, rows))
print(len(valid_rows))
