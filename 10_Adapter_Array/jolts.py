#!/usr/bin/env python3

from collections import defaultdict
import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
adapters = list(map(int, open(filename).read().strip().split("\n")))

adapters.append(0)
adapters.sort()
adapters.append(adapters[-1] + 3)


def part_one():
    diffs = defaultdict(int)
    prev = adapters[0]
    for cur in adapters[1:]:
        diffs[cur - prev] += 1
        prev = cur

    print(diffs)
    print(diffs[1] * diffs[3])


def part_two():

    # this list is probably way too long, whatever
    dp = [None] * len(adapters)

    def configurations(length):
        # Number of possible configurations of left-out
        # adapters, all with differences one to each previous one

        if length < 0:
            return 0
        if length < 2:
            return 1

        if dp[length] is not None:
            return dp[length]

        x = (
            # Cases:
            #  - first stays in
            configurations(length - 1)
            #  - first goes out
            #   - second stays in
            + configurations(length - 2)
            #   - second goes out, therefore third has to stay in
            + configurations(length - 3)
        )
        dp[length] = x
        return x

    three_splits = [0]
    for i in range(1, len(adapters)):
        if adapters[i] - adapters[i - 1] == 3:
            three_splits.append(i)
    arrangements = 1
    for i in range(len(three_splits) - 1):
        start = three_splits[i]
        end = three_splits[i + 1]
        arrangements *= configurations(end - start - 1)

    print(arrangements)


part_one()
part_two()
