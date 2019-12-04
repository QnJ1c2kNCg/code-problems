def respect(password):
    contains_adjacent = False
    ordered = True
    previous_digit = -1
    for d in [int(x) for x in str(password)]:
        if previous_digit == d:
            contains_adjacent = True
        if d < previous_digit:
            return False
        previous_digit = d

    return contains_adjacent

def part_1():
    password_set = set()
    for i in range(359282, 820401 + 1):
        if respect(i):
            password_set.add(i)
    return password_set


def part_2(password_set):
    rv = set()
    for password in password_set:
        occurence = dict()
        for d in [int(x) for x in str(password)]:
            if d in occurence:
                occurence[d] += 1
            else:
                occurence[d] = 1
        for v in occurence.values():
            if v == 2:
                rv.add(password)
                break;
    return len(rv)




print('Part 1: ', len(part_1()))
print('Part 2: ', part_2(part_1()))
