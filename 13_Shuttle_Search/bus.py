#!/usr/bin/env python3

import sys
import operator
from functools import reduce

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
data = open(filename).read().strip().split("\n")


def part_one():
    current_time = int(data[0])
    bus_ids = [int(x) for x in data[1].split(",") if x != "x"]
    time_passed_since_last_bus = [
        ((current_time - 1) % bus) + 1 for bus in bus_ids]
    time_left_till_bus_arrives = [bus_id - time_passed for bus_id,
                                  time_passed in zip(bus_ids, time_passed_since_last_bus)]
    print(time_left_till_bus_arrives)

    next_bus = min(enumerate(time_left_till_bus_arrives), key=lambda t: t[1])
    print(next_bus)
    print(bus_ids[next_bus[0]] * next_bus[1])


def egcd(a, b):
    if a == 0:
        return (b, 0, 1)
    else:
        gcd, x, y = egcd(b % a, a)
        return (gcd, y - (b // a) * x, x)


def part_two(num_str):
    # Chinese Remainder Theorem
    # see https://de.wikipedia.org/wiki/Chinesischer_Restsatz

    ai_mi = [(int(m) - i, int(m))
             for i, m in enumerate(num_str.split(",")) if m != "x"]
    ai = [a for a, m in ai_mi]
    mi = [m for a, m in ai_mi]
    M = reduce(operator.mul, mi)
    Mi = [M // m for m in mi]

    si = [egcd(m, M)[2] for m, M in zip(mi, Mi)]
    ei = [s * M for s, M in zip(si, Mi)]

    x = sum(a * e for a, e in zip(ai, ei))

    # x is a solution, for this task the smallest non-negative solution is needed, so adjust
    while x < 0:
        x += M
    x %= M

    return x


print("Part one:")
part_one()
print()

print("Part two:")

# Testing
EXAMPLES = [
    ("7,13,x,x,59,x,31,19", 1068781),
    ("17,x,13,19", 3417),
    ("67,7,59,61", 754018),
    ("67,x,7,59,61", 779210),
    ("67,7,x,59,61", 1261476),
    ("1789,37,47,1889", 1202161486)
]

for s, expected in EXAMPLES:
    res = part_two(s)
    print(res, expected, res == expected)

# Actual solution
print()
print(part_two(data[1]))
