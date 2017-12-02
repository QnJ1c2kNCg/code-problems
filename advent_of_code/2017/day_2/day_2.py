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

def part_2(input_file_name):
    """ Part 2 """
    with open(input_file_name) as f:
        ret_sum = 0
        for line in f:
            line_list = [int(n) for n in line.split()]
            list_size = len(line_list)
            for i in range(list_size):
                for j in range(i + 1, list_size):
                    if line_list[i] % line_list[j] == 0:
                        ret_sum += line_list[i] // line_list[j]
                    elif line_list[j] % line_list[i] == 0:
                        ret_sum += line_list[j] // line_list[i]

        return ret_sum


print("Part one: {}".format(part_1(INPUT_FILE_NAME)))
print("Part two: {}".format(part_2(INPUT_FILE_NAME)))
