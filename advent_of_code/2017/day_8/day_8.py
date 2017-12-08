""" Day 8 """

def is_valid_condition(condition_register, condition, condition_value):
    if condition == '<':
        return registers.get(condition_register, 0) < int(condition_value)
    elif condition == '>':
        return registers.get(condition_register, 0) > int(condition_value)
    elif condition == '>=':
        return registers.get(condition_register, 0) >= int(condition_value)
    elif condition == '<=':
        return registers.get(condition_register, 0) <= int(condition_value)
    elif condition == '==':
        return registers.get(condition_register, 0) == int(condition_value)
    elif condition == '!=':
        return registers.get(condition_register, 0) != int(condition_value)

def part_1():
    """ Part 1 """
    with open('input.txt') as f:
        for line in f:
            words = line.split()
            register = words[0]
            value = int(words[2])
            if words[1] == 'dec':
                value *= -1
            condition_register = words[4]
            condition = words[5]
            condition_value = words[6]
            if is_valid_condition(condition_register, condition, condition_value):
                registers[register] = registers.get(register, 0) + value

        return max(registers.values())

def part_2():
    """ Part 2 """
    with open('input.txt') as f:
        all_time_max = 0
        for line in f:
            words = line.split()
            register = words[0]
            value = int(words[2])
            if words[1] == 'dec':
                value *= -1
            condition_register = words[4]
            condition = words[5]
            condition_value = words[6]
            if is_valid_condition(condition_register, condition, condition_value):
                registers[register] = registers.get(register, 0) + value

            if registers and all_time_max < max(registers.values()):
                all_time_max = max(registers.values())

        return all_time_max

registers = dict()
print('Part one: ', part_1())
registers = dict()
print('Part two: ', part_2())
