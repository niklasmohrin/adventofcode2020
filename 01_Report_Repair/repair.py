#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
data = set(map(int, open(filename).read().strip().split("\n")))

# Part one
for n in data:
    if 2020 - n in data:
        print(f"Found: {n} and {2020 - n} with product {n * (2020 - n)}")

# Part two
for n1 in data:
    for n2 in data:
        if 2020 - n1 - n2 in data:
            print(
                f"Found: {n1}, {n2} and {2020 - n1 - n2} with product {n1 * n2 * (2020 - n1 - n2)}"
            )
