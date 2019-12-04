def calculate_fuel(weight, recursion=False):
    fuel_needed = weight // 3 - 2 if weight // 3 - 2 > 0 else 0
    if not recursion:
        return fuel_needed
    if fuel_needed == 0:
        return 0
    return fuel_needed + calculate_fuel(fuel_needed, recursion=True)

def part_1():
    total = 0
    with open('input.txt') as f:
        for line in f:
            total += calculate_fuel(int(line))
    return total

def part_2():
    total = 0
    with open('input.txt') as f:
        for line in f:
            total += calculate_fuel(int(line), recursion=True)
    return total


print('Part 1: ', part_1())
print('Part 2: ', part_2())
