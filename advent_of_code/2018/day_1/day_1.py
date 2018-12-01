def part_1():
    total = 0
    with open('input.txt') as f:
        for line in f:
            if line.startswith('-'):
                total -= int(line[1:])
            else:
                total += int(line[1:])
    return total

print('The total is: ', part_1())