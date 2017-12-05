""" Day 5 """

FILE_NAME = "input.txt"

def create_list(file_name):
    """ Transorms the input to a list of ints """
    with open(file_name) as f:
        steps_list = list()
        for line in f:
            steps_list.append(int(line))
        return steps_list


def part_1(steps):
    """ Part 1 """
    current_index = 0
    total_steps = 0
    while current_index >= 0 and current_index < len(steps):
        step = steps[current_index]
        steps[current_index] += 1
        current_index += step
        total_steps += 1
    return total_steps

def part_2(steps):
    """ Part 2 """
    current_index = 0
    total_steps = 0
    while current_index >= 0 and current_index < len(steps):
        step = steps[current_index]
        if step >= 3:
            steps[current_index] -= 1
        else:
            steps[current_index] += 1
        current_index += step
        total_steps += 1
    return total_steps

print("Part one: ", part_1(create_list(FILE_NAME)))
print("Part two: ", part_2(create_list(FILE_NAME)))
