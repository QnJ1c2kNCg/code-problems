""" Day 9 """

def part_1():
    """ Part 1 """
    with open('input.txt') as f:
        total_score = 0
        is_in_garbage = False
        current_block_score = 0
        ignore_next = False

        for c in f.readline():
            if ignore_next:
                ignore_next = False
                continue
            if c == '!':
                ignore_next = True
            elif c == '<':
                is_in_garbage = True
            elif c == '>':
                is_in_garbage = False
            elif c == '{' and not is_in_garbage:
                current_block_score += 1
            elif c == '}' and not is_in_garbage:
                total_score += current_block_score
                current_block_score -= 1
        return total_score

def part_2():
    """ Part 2 """
    with open('input.txt') as f:
        is_in_garbage = False
        ignore_next = False
        characters_in_garbage = 0

        for c in f.readline():
            if ignore_next:
                ignore_next = False
                continue
            if c == '!':
                ignore_next = True
            elif c == '<':
                if is_in_garbage:
                    characters_in_garbage += 1
                is_in_garbage = True
            elif c == '>':
                is_in_garbage = False
            elif is_in_garbage:
                characters_in_garbage += 1

        return characters_in_garbage

print('Part one: ', part_1())
print('Part two: ', part_2())
