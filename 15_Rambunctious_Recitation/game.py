#!/usr/bin/env python3

import sys

MY_INPUT = "14,1,17,0,3,20"
# Part one
# TURN_IN_QUESTION = 2020

# Part two
# Terminates after ~30 seconds
TURN_IN_QUESTION = 30000000

data = sys.argv[1] if len(sys.argv) > 1 else MY_INPUT
starting_numbers = [int(x) for x in data.split(",")]

last_spoken = dict()
last_number = None

for turn in range(1, TURN_IN_QUESTION + 1):
    if turn - 1 < len(starting_numbers):
        number_spoken = starting_numbers[turn - 1]
    else:
        if last_number in last_spoken and last_spoken[last_number][0] is not None:
            number_spoken = (turn - 1) - last_spoken[last_number][0]
        else:
            number_spoken = 0

    if number_spoken in last_spoken:
        last_spoken[number_spoken] = (last_spoken[number_spoken][1], turn)
    else:
        last_spoken[number_spoken] = (None, turn)
    last_number = number_spoken

print(last_number)
