def distance(coord_1, coord_2):
    return abs(coord_1[0] - coord_2[0]) + abs(coord_1[1] - coord_2[1])

def belongs_to(current_coord, coords):
    multiple_same_dist = False
    current_min_dist = 10000000
    min_coord = None
    for coord in coords:
        dist = distance(coord, current_coord)
        if dist < current_min_dist:
            current_min_dist = dist
            min_coord = coord
            multiple_same_dist = False
        elif dist == current_min_dist:
            multiple_same_dist = True
            min_coord = None

    return min_coord


def check_grid(coords, grid_size=500):
    territory = dict()
    for x in range(-grid_size, grid_size):
        for y in range(-grid_size, grid_size):
            belongs_to_ = belongs_to((x, y), coords)
            if belongs_to_:
                territory[belongs_to_] = territory.get(belongs_to_, 0) + 1

    return territory


def part_1():
    # 1. Create all my coords
    with open('input.txt', 'r') as f:
        coords = [tuple(map(int, x.split(','))) for x in f.read().splitlines()]

    # 2. Create a large grid
    # 3. For each cell in the grid, check who it belongs to
    #    have a counter for every cell that increments 
    small_territory = check_grid(coords, 400)

    # 4. Create a *bigger* grid
    # Do the same thing
    # If the size changed then it's infinite, so discard, otherwise, pick the biggest
    big_territory = check_grid(coords, 800)

    # Drop all infinite zones
    intersection = dict(small_territory.items() & big_territory.items())

    return max(intersection.values())

def part_2():
    # 1. Create all my coords
    with open('input.txt', 'r') as f:
        coords = [tuple(map(int, x.split(','))) for x in f.read().splitlines()]

    region_size = 0
    for x in range(-4000, 4000):
        for y in range(-4000, 4000):
            total_dist = 0
            for coord in coords:
                total_dist += distance((x, y), coord)
            if total_dist < 10000:
                region_size += 1

    return region_size


print('Part 1:', part_1())
print('Part 2:', part_2())