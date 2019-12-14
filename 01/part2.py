#!/usr/bin/env python3

import math

def fuel_required(mass):
    fuel = math.floor(mass / 3) - 2
    if fuel < 9:
        return fuel
    else:
        return fuel + fuel_required(fuel)

total_fuel = 0

with open("input.txt") as f:
    for line in f:
        if line.rstrip() != "":
            total_fuel += fuel_required(int(line))

print("Total fuel: {}".format(total_fuel))