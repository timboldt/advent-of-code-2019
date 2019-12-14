#!/usr/bin/env python3

import math

def fuel_required(mass):
    return math.floor(mass / 3) - 2

total_fuel = 0

with open("input/01.txt") as f:
    for line in f:
        if line.rstrip() != "":
            total_fuel += fuel_required(int(line))

print("Total fuel: {}".format(total_fuel))
