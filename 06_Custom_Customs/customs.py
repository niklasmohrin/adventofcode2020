#!/usr/bin/env python3

import sys
from string import ascii_lowercase

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
groups = open(filename).read().strip().split("\n\n")


# Part one
# def group_count(group):
#     return len(set(group) & set(ascii_lowercase))

# Part two
def group_count(group):
    res = set(ascii_lowercase)
    for person in group.split("\n"):
        res &= set(person)
    return len(res)


print(sum(map(group_count, groups)))
