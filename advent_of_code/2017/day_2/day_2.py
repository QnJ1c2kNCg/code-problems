""" Day 2 """

INPUT_FILE_NAME = "input.txt"

def part_1(input_file_name):
    """ Part 1 """
    with open(input_file_name) as f:
        ret_sum = 0
        for line in f:
            line_list = [int(n) for n in line.split()]
            max_number = max(line_list)
            min_number = min(line_list)
            ret_sum += max_number - min_number

        return ret_sum

print("Part one: {}".format(part_1(INPUT_FILE_NAME)))
