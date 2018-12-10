import re
import matplotlib.pyplot as plt

def find_smallest_size(lines):
    min_i = 0
    min_size = 1000000000000
    for i in range(36000):
        minx = min(x + i * vx for (x, y, vx, vy) in lines)
        maxx = max(x + i * vx for (x, y, vx, vy) in lines)
        miny = min(y + i * vy for (x, y, vx, vy) in lines)
        maxy = max(y + i * vy for (x, y, vx, vy) in lines)
        size = (maxx - minx) * (maxy - miny)
        if size < min_size:
            min_size = size
            min_i = i
    return (min_i, min_size)

def part_1_2():
    with open('input.txt', 'r') as f:
        numbers_re = re.compile('-?\d+')  # Find all the numbers within the string
        lines = f.read().splitlines()
        lines = [[int(x) for x in  numbers_re.findall(line)] for line in lines]

    target_i, _ = find_smallest_size(lines)

    for line in lines:
        x = line[0] + target_i * line[2]
        y = line[1] + target_i * line[3]
        plt.scatter(y, x)
        plt.xticks(list(range(100, 200, 10)))
    plt.show()

    return target_i

print('Part 1: look at the plot')
print('Part 2:', part_1_2())