import re

# Add field on program to save the children weight
INPUT_FILE_NAME = "input.txt"

class Program:
    def __init__(self, weight, name, children = []):
        self.weight = weight
        self.name = name
        self.children = children
        self.incident_edges = 0
    
    def get_children(self, programs):
        children_programs = []
        for child in self.children:
            children_programs.append(programs[child])
        return children_programs

    def get_total_weight(self, programs):
        total_weight = self.weight
        for child in self.get_children(programs):
            total_weight += child.get_total_weight(programs)
        return total_weight

def create_program_dict(file_name):
    """ Creates a list of `Program` """
    with open(file_name) as f:
        programs = dict()
        for line in f:
            weight = re.search(r"\((\S+)\)", line).group(1)
            name = line.split()[0]
            children = []
            if line.find('>') != -1:
                str_after_arrow = line.split('> ')[1].replace('\n', '')
                children = str_after_arrow.split(', ')
            programs[name] = Program(weight, name, children)
        return programs

def part_1(programs):
    """ Part 1 """
    for name, program in programs.items():
        for child in program.children:
            programs[child].incident_edges += 1

    for name, program in programs.items():
        if program.incident_edges == 0:
            return program

def find_outlier(programs, program):
    programs_dict = dict()
    for child in program.get_children(programs):
        weight = child.get_total_weight(programs)
        print(child.name)
        print(weight)
        programs_dict[weight] = (programs_dict.get(weight, 0) + 1, child)

    for key, value in programs_dict.items():
        if value == 1:
            return value[1]

    return None

def find_parent(programs, program_to_query):
    print("program to query: ", program_to_query.name)
    for name, program in programs.items():
        for child in program.children:
            if child == program_to_query.name:
                return program



def part_2(programs, root_program):
    """ Part 2 """
    previous_program = None
    current_program = root_program
    while find_outlier(programs, current_program) != None:
        print('hey')
        previous_program = root_program
        root_program = find_outlier(programs, current_program)

    print(previous_program)
    program_to_modify = previous_program
    parent_program = find_parent(programs, program_to_modify)
    print(parent_program.name)
    for child in parent_program.children:
        print(child)








programs = create_program_dict(INPUT_FILE_NAME)
print("Part one: ", part_1(programs).name)
print("Part two: ", part_2(create_program_dict(INPUT_FILE_NAME), part_1(programs)))
