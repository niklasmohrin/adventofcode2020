#!/usr/bin/env python3

from functools import reduce
import operator
import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
rows = open(filename).read().strip().split("\n")
width = len(rows[0])

TREE = '#'


def trajectory_for_slope(dx, dy=1):
    x = 0
    counter = 0

    for y in range(0, len(rows), dy):
        if rows[y][x] == TREE:
            counter += 1
        x += dx
        x %= width

    return counter


# Part one
# print(trajectory_for_slope(3))

# Part two
slopes = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
]

encounters = [trajectory_for_slope(dx, dy) for (dx, dy) in slopes]
print(reduce(operator.mul, encounters))
