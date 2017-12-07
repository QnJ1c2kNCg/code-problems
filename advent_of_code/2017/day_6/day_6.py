""" Day 6 """

INPUT = '0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11'

def part_1(input_str):
    """ Part 1 """
    current_list = [int(x) for x in input_str.split()]
    seens_states = set()

    redistribution_cycles = 0

    while "".join(str(x) for x in current_list) not in seens_states:
        redistribution_cycles += 1
        seens_states.add("".join(str(x) for x in current_list))
        points_to_distribute = max(current_list)
        current_index = current_list.index(points_to_distribute)
        current_list[current_index] = 0
        while points_to_distribute > 0:
            current_index += 1
            points_to_distribute -= 1
            current_list[current_index % len(current_list)] += 1

    return redistribution_cycles

def part_2(input_str):
    """ Part 2 """
    current_list = [int(x) for x in input_str.split()]
    seens_states = list()

    redistribution_cycles = 0

    while "".join(str(x) for x in current_list) not in seens_states:
        redistribution_cycles += 1
        seens_states.append("".join(str(x) for x in current_list))
        points_to_distribute = max(current_list)
        current_index = current_list.index(points_to_distribute)
        current_list[current_index] = 0
        while points_to_distribute > 0:
            current_index += 1
            points_to_distribute -= 1
            current_list[current_index % len(current_list)] += 1

    first_index = seens_states.index("".join(str(x) for x in current_list))
    return redistribution_cycles - first_index


print('Part one: ', part_1(INPUT))
print('Part two: ', part_2(INPUT))
