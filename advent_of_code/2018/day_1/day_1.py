def part_1():
    total = 0
    with open('input.txt') as f:
        for line in f:
            if line.startswith('-'):
                total -= int(line[1:])
            else:
                total += int(line[1:])
    return total

def part_2():
    lines = []
    with open('input.txt') as f:
        lines = f.readlines()

    total = 0
    frequencies = set()
    while True:
        for line in lines:
            if line.startswith('-'):
                total -= int(line[1:])
            else:
                total += int(line[1:])

            if total in frequencies:
                return total
            else:
                frequencies.add(total)


print('Part 1 -> The total is: ', part_1())
print('Part 2 -> The firs frequency hit twice is: ', part_2())