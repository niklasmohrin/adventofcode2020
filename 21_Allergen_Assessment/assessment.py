#!/usr/bin/env python3

import sys

filename = sys.argv[1] if len(sys.argv) > 1 else "input"
data = open(filename).read().strip().split("\n")

info = []
all_products = set()
all_allergens = set()
for line in data:
    products, allergens = line[:-1].split(" (contains ")
    products = products.split()
    allergens = allergens.split(", ")
    all_products |= set(products)
    all_allergens |= set(allergens)
    info.append((products, allergens))

food_of = dict()

cleaned_info = info.copy()

progress = True
while progress:
    progress = False
    for al in all_allergens:
        foods = all_products.copy()
        for (ps, alls) in cleaned_info:
            if al in alls:
                foods &= set(ps)
        if len(foods) == 1:
            progress = True
            food = list(foods)[0]
            food_of[al] = food
            for i in cleaned_info:
                if food in i[0]:
                    i[0].remove(food)
                if al in i[1]:
                    i[1].remove(al)

healthy_food = all_products - set(food_of.values())

# Part one
count = 0
for ps, _ in info:
    count += len(set(ps) & healthy_food)
print(count)

# Part two
print(",".join(map(lambda t: t[1], sorted(
    food_of.items(), key=lambda t: t[0]))))
