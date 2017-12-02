""" Day 2 """

import functools
import operator

INPUT_FILE_NAME = "input.txt"

class Box:
    """ Represents a box to be wrapped """

    def __init__(self, dimensions):
        self.dimensions_list = sorted([int(x) for x in dimensions.split('x')])
        self.l = self.dimensions_list[0]
        self.w = self.dimensions_list[1]
        self.h = self.dimensions_list[2]

    def get_total_area(self):
        """ Returns the total area of the box """
        return 2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l

    def get_area_smallest_side(self):
        """ Returns the area of the smallest side """
        sides_area = [self.l*self.w, self.w*self.h, self.h*self.l]
        return min(sides_area)

    def get_total_paper_needed(self):
        """ Returns the total paper needed """
        return self.get_total_area() + self.get_area_smallest_side()

    def get_smallest_perimeter(self):
        """ Returns the smallest perimeter of the box """
        # The list is sorted
        return 2 * self.dimensions_list[0] + 2 * self.dimensions_list[1]

    def get_cubic_volume(self):
        """ Returns the cubic volume of the box """
        return functools.reduce(operator.mul, self.dimensions_list)

    def get_total_ribbon_lenght(self):
        """ Returns the total ribbon needed """
        return self.get_smallest_perimeter() + self.get_cubic_volume()


def part_1(file_name):
    """ Part 1 """
    with open(file_name) as f:
        paper_needed = 0
        for line in f:
            box = Box(line)
            paper_needed += box.get_total_paper_needed()
        return paper_needed

def part_2(file_name):
    """ Part 2 """
    with open(file_name) as f:
        ribbon_needed = 0
        for line in f:
            box = Box(line)
            ribbon_needed += box.get_total_ribbon_lenght()
        return ribbon_needed

print('Part one: ', part_1(INPUT_FILE_NAME))
print('Part two: ', part_2(INPUT_FILE_NAME))
