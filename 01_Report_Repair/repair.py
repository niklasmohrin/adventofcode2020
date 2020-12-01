#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
data = set(map(int, open(filename).read().strip().split("\n")))

for n1 in data:
    for n2 in data:
        if 2020 - n1 - n2 in data:
            print(
                f"Found: {n1}, {n2} and {2020 - n1 - n2} with product {n1 * n2 * (2020 - n1 - n2)}"
            )
