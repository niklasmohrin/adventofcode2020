#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
data = open(filename).read().strip().split("\n")
