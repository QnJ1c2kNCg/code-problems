""" Day 4 """

FILE_NAME = "input.txt"

def part_1(file_name):
    """ Part 1 """
    with open(file_name) as f:
        valid = 0
        for line in f:
            valid += 1
            words = set()
            for word in line.split():
                if word in words:
                    valid -= 1
                    break
                words.add(word)
        return valid

def part_2(file_name):
    """ Part 2 """
    with open(file_name) as f:
        valid = 0
        for line in f:
            valid += 1
            words = set()
            for word in line.split():
                sorted_word = ''.join(sorted(word))
                if sorted_word in words:
                    valid -= 1
                    break
                words.add(sorted_word)
        return valid

print("Part one: ", part_1(FILE_NAME))
print("Part two: ", part_2(FILE_NAME))
