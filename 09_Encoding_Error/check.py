#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
nums = list(map(int, open(filename).read().strip().split("\n")))

RANGE = 25


def summands(s, ns):
    for i, n1 in enumerate(ns):
        for n2 in ns[i+1:]:
            if n1 + n2 == s:
                return (n1, n2)


invalid_number = None

for i, n in list(enumerate(nums))[RANGE:]:
    if summands(n, nums[i-RANGE:i]) is None:
        invalid_number = n
        break


# Part one
print("Something doesn't add up", invalid_number)

# Part two
lo = hi = 0
s = 0
for i, n in enumerate(nums):
    hi = i
    s += n
    while s > invalid_number:
        s -= nums[lo]
        lo += 1
    if s == invalid_number:
        print("Range found:", lo, hi, min(
            nums[lo:hi+1]) + max(nums[lo:hi+1]), sum(nums[lo:hi+1]))
        break
