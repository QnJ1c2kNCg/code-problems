""" Day 3 """

"""
NOTES:
- bottom right corner = odd number square

"""

def find_indices(number):
    current_index = 1
    current_value = current_index ** 2
    # I can do a better loop
    while current_value <= number:
        current_index += 2
        current_value = current_index ** 2

    current_index -= 2
    current_value = current_index ** 2
    current_indices = [current_index // 2, -(current_index // 2)]

    if current_value == number:
        return current_indices

    current_indices[0] += 1
    current_value +=1
    if current_value == number:
        return current_indices

    for i in range(current_index + 1):
        current_indices[1] += 1
        current_value += 1

        if current_value == number:
            return current_indices

    for i in range(current_index + 1):
        current_indices[0] -= 1
        current_value += 1

        if current_value == number:
            return current_indices

    for i in range(current_index + 1):
        current_indices[1] -= 1
        current_value += 1

        if current_value == number:
            return current_indices

    for i in range(current_index + 1):
        current_indices[0] += 1
        current_value += 1

        if current_value == number:
            return current_indices

    return None

def manhattan_distance(arr):
    return abs(arr[0]) + abs(arr[1])

"""
def foo(number):
    indices = [0, 0]
    current_number = 1
    while current_number != number:
        current_number += 1
        """


print(manhattan_distance(find_indices(325489)))

# 880