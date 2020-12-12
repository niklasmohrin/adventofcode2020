#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
data = open(filename).read().strip().split("\n")
moves = [(x[0], int(x[1:])) for x in data]

# X goes right, Y goes down
NORTH = (0, -1)
SOUTH = (0, 1)
WEST = (-1, 0)
EAST = (1, 0)
direction = EAST

# Part one
# x = 0
# y = 0
#
# def turn_right(angle):
#     global direction
#
#     for _ in range(angle // 90):
#         if direction == EAST:
#             direction = SOUTH
#         elif direction == SOUTH:
#             direction = WEST
#         elif direction == WEST:
#             direction = NORTH
#         elif direction == NORTH:
#             direction = EAST
#         else:
#             raise (direction, angle)
#
#
# for (cmd, mag) in moves:
#     if cmd in "NSWE":
#         dx, dy = ({"N": NORTH, "S": SOUTH, "W": WEST, "E": EAST})[cmd]
#         x += dx * mag
#         y += dy * mag
#     elif cmd in "LR":
#         angle = mag
#         if cmd == "L":
#             angle = 360 - angle
#         turn_right(angle)
#     elif cmd == "F":
#         dx, dy = direction
#         x += dx * mag
#         y += dy * mag
#     else:
#         raise (cmd, mag)
#
# print(x, y)
# print(abs(x) + abs(y))


ship_x = 0
ship_y = 0
# waypoint position is relative to ship
waypoint_x = 10
waypoint_y = -1


def rotate_waypoint_right(angle):
    global waypoint_x
    global waypoint_y

    for _ in range(angle // 90):
        temp_y = waypoint_y
        waypoint_y = waypoint_x
        waypoint_x = -temp_y


for (cmd, mag) in moves:
    if cmd in "NSWE":
        dx, dy = ({"N": NORTH, "S": SOUTH, "W": WEST, "E": EAST})[cmd]
        waypoint_x += dx * mag
        waypoint_y += dy * mag
    elif cmd in "LR":
        angle = mag
        if cmd == "L":
            angle = 360 - angle
        rotate_waypoint_right(angle)
    elif cmd == "F":
        ship_x += waypoint_x * mag
        ship_y += waypoint_y * mag
    else:
        raise (cmd, mag)

print(ship_x, ship_y)
print(waypoint_x, waypoint_y)
print(abs(ship_x) + abs(ship_y))
