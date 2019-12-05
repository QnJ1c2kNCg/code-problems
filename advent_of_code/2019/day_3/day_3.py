import copy
import sys

def trace(instructions):
    rv = set()
    current_pos = (0, 0)
    for instruction in instructions:
        if instruction.startswith('R'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0] + 1, current_pos[1])
                rv.add(current_pos)
        elif instruction.startswith('L'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0] - 1, current_pos[1])
                rv.add(current_pos)
        elif instruction.startswith('D'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0], current_pos[1] - 1)
                rv.add(current_pos)
        elif instruction.startswith('U'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0], current_pos[1] + 1)
                rv.add(current_pos)
        else:
            print("Error!")

    return rv

def part_1():
    with open('input.txt') as f:
        lines = f.readlines()
        wire_1 = trace(lines[0].split(','))
        wire_2 = trace(lines[1].split(','))
        intersections = wire_1 & wire_2

        min_distance = sys.maxsize
        for intersection in intersections:
            distance = abs(intersection[0]) + abs(intersection[1])
            if distance < min_distance:
                min_distance = distance

    return min_distance

def walk(steps, coords):
    current_pos = (0, 0)
    dist = 0
    for instruction in steps:
        if instruction.startswith('R'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0] + 1, current_pos[1])
                dist += 1
                if current_pos == coords:
                    return dist
        elif instruction.startswith('L'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0] - 1, current_pos[1])
                dist += 1
                if current_pos == coords:
                    return dist
        elif instruction.startswith('D'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0], current_pos[1] - 1)
                dist += 1
                if current_pos == coords:
                    return dist
        elif instruction.startswith('U'):
            for _ in range(0, int(instruction[1:])):
                current_pos = (current_pos[0], current_pos[1] + 1)
                dist += 1
                if current_pos == coords:
                    return dist
        else:
            print("Error!")



def part_2():
    with open('input.txt') as f:
        lines = f.readlines()
        steps_1 = lines[0].split(',')
        steps_2 = lines[1].split(',')
        wire_1 = trace(steps_1)
        wire_2 = trace(steps_2)
        intersections = wire_1 & wire_2
        min_distance = sys.maxsize
        for intersection in intersections:
            total = walk(steps_1, intersection) + walk(steps_2, intersection)
            if total < min_distance:
                min_distance = total

        return min_distance



print('Part 1: ', part_1())
print('Part 2: ', part_2())
