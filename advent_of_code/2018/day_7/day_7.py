import re
from collections import defaultdict

def step_duration(step):
    return 60 + ord(step) - ord('A') + 1

def part_1():
    steps_re = re.compile('Step (.) must be finished before step (.) can begin.')
    with open('input.txt', 'r') as f:
        steps = [steps_re.findall(x)[0] for x in f.read().splitlines()]

    dependencies = dict()
    for step in steps:
        dependencies[step[0]] = list()
        dependencies[step[1]] = list()

    # Build a dict of dependencies
    for step in steps:
        lst = dependencies[step[1]]
        lst.append(step[0])

    ret_str = str()
    # Loop on the one that don't have any dependencies
    while len(dependencies) > 0:
        # Find the letters that don't have an dependencies
        zero_dep = sorted([k for k, v in dependencies.items() if len(v) == 0])
        step_to_remove = zero_dep[0]
        ret_str += step_to_remove
        dependencies.pop(step_to_remove)
        for v in dependencies.values():
            if step_to_remove in v: v.remove(step_to_remove)

    return ret_str

def part_2():
    steps_re = re.compile('Step (.) must be finished before step (.) can begin.')
    with open('input.txt', 'r') as f:
        steps = [steps_re.findall(x)[0] for x in f.read().splitlines()]

    dependencies = dict()
    for step in steps:
        dependencies[step[0]] = list()
        dependencies[step[1]] = list()

    # Build a dict of dependencies
    for step in steps:
        lst = dependencies[step[1]]
        lst.append(step[0])

    current_time = 0
    running_tasks = dict()
    available_worker = 5
    # Loop on the one that don't have any dependencies
    while len(dependencies) > 0:
        # Check if some tasks are done
        tasks_done = list()
        for k, v in running_tasks.items():
            if current_time >= v:
                # Remove from list
                dependencies.pop(k)
                tasks_done.append(k)
                for v in dependencies.values():
                    if k in v: v.remove(k)
        for task_done in tasks_done:
            running_tasks.pop(task_done)
            available_worker += 1

        # Find the letters that don't have an dependencies
        zero_dep = sorted([k for k, v in dependencies.items() if len(v) == 0])
        zero_dep = [x for x in zero_dep if x not in running_tasks]
        while available_worker > 0 and len(zero_dep) > 0:
            available_worker -= 1
            task_to_remove = zero_dep.pop(0)
            running_tasks[task_to_remove] = current_time + step_duration(task_to_remove)
        
        current_time += 1

    return current_time - 1

print('Part 1:', part_1())
print('Part 2:', part_2())