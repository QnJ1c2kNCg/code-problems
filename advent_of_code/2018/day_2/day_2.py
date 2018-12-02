from collections import Counter

def countains_same_letters(counter, goal):
    for k, v in counter.items():
        if v == goal:
            return True

    return False

def part_1():
    two_multiplier = 0
    three_multiplier = 0
    with open('input.txt', 'r') as f:
        for line in f:
            counter = Counter(line)
            if countains_same_letters(counter, 2):
                two_multiplier += 1
            if countains_same_letters(counter, 3):
                three_multiplier += 1

    return two_multiplier * three_multiplier

def differ_by_one(str_1, str_2):
    lenght = len(str_1)
    common_str = str()
    for i in range(0, lenght):
        if str_1[i] == str_2[i]:
            common_str += str_1[i]

    return common_str if lenght - len(common_str) == 1 else None


def part_2():
    with open('input.txt', 'r') as f:
        lines = f.read().splitlines()

        for line in lines:
            for other_line in lines:
                diff = differ_by_one(line, other_line)
                if diff:
                    return diff


print('Part 1: ', part_1())
print('Part 2: ', part_2())