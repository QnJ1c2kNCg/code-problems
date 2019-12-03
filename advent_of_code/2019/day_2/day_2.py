import copy

def part_1():
    numbers = list()
    with open('input.txt') as f:
        index = 0
        numbers = list(map(int, f.readlines()[0].split(',')))
        while numbers[index] != 99:
            if numbers[index] == 1:
                numbers[numbers[index + 3]] = numbers[numbers[index + 1]] + numbers[numbers[index + 2]]
            elif numbers[index] == 2:
                numbers[numbers[index + 3]] = numbers[numbers[index + 1]] * numbers[numbers[index + 2]]
            else:
                print('Error! Got {}'.format(numbers[index]))
            index += 4
    return numbers[0]

def part_2():
    original_numbers = list()
    with open('input.txt') as f:
        original_numbers = list(map(int, f.readlines()[0].split(',')))

    for i in range(0, 100):
        for j in range(0, 100):
            index = 0
            numbers = copy.deepcopy(original_numbers)
            numbers[1] = i
            numbers[2] = j
            while numbers[index] != 99:
                if numbers[index] == 1:
                    numbers[numbers[index + 3]] = numbers[numbers[index + 1]] + numbers[numbers[index + 2]]
                elif numbers[index] == 2:
                    numbers[numbers[index + 3]] = numbers[numbers[index + 1]] * numbers[numbers[index + 2]]
                else:
                    print('Error! Got {}'.format(numbers[index]))
                index += 4
            if numbers[0] == 19690720:
                return 100 * i + j
    return -1


print('Part 1: ', part_1())
print('Part 2: ', part_2())
