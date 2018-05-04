import re


def find_group(id='0'):
    instructions = open('input.txt').readlines()
    program_set = set()
    processed_program = set()
    program_set.add(id)
    has_changed = True
    while has_changed:
        has_changed = False
        for instruction in instructions:
            ids = re.findall(r'(\d+)', instruction)
            if ids[0] in program_set and ids[0] not in processed_program:
                program_set.update(ids[1:])
                processed_program.add(ids[0])
                has_changed = True

    return program_set

def part_1():
    return len(find_group('0'))

def part_2():
    n_groups = 0
    possible_programs = list()
    with open('input.txt', 'r') as f:
        for line in f:
            possible_programs.append(line.split(None, 1)[0]) # add only first word

    while len(possible_programs) > 0:
        group = find_group(possible_programs[0])
        n_groups += 1
        possible_programs = [x for x in possible_programs if x not in group]

    return n_groups

#print(part_1())
print(part_2())