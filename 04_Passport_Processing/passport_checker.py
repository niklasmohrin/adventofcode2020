#!/usr/bin/env python3

import re
import sys

filename = sys.argv[1] if len(sys.argv) == 2 else "input"
passports = open(filename).read().strip().split("\n\n")


RULES = {
    # (Birth Year)
    "byr": lambda x: len(x) == 4 and x.isnumeric() and 1920 <= int(x) <= 2002,
    # (Issue Year)
    "iyr": lambda x: len(x) == 4 and x.isnumeric() and 2010 <= int(x) <= 2020,
    # (Expiration Year)
    "eyr": lambda x: len(x) == 4 and x.isnumeric() and 2020 <= int(x) <= 2030,
    # (Height)
    "hgt": lambda x: x[:-2].isnumeric() and (
        (x[-2:] == "cm" and 150 <= int(x[:-2]) <= 193)
        or
        (x[-2:] == "in" and 59 <= int(x[:-2]) <= 76)
    ),
    # (Hair Color)
    "hcl": lambda x: re.match(r"^#[0-9a-f]{6}$", x) is not None,
    # (Eye Color)
    "ecl": lambda x: x in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
    # (Passport ID)
    "pid": lambda x: re.match(r"^[0-9]{9}$", x) is not None,
    # "cid",  # (Country ID)
}


def passport_is_valid(passport):
    fields = [field.split(":") for field in passport.split()]
    fields = {k: v for (k, v) in fields}
    return all(key in fields and RULES[key](fields[key]) for key in RULES)


print(len(list(filter(passport_is_valid, passports))))
