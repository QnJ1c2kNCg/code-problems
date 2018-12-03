import re

def part_1_2():
    with open('input.txt', 'r') as f:
        coords_used = set()
        coords_overlap = set()
        p = re.compile('#(\d+) @ (\d+,\d+): (\d+x\d+)')
        lines = f.readlines()
        for line in lines:
            group = p.findall(line)[0]
            elf_id = group[0]
            start_coords = [int(x) for x in group[1].split(',')]
            dims = [int(x) for x in group[2].split('x')]
            for x in range(start_coords[0], start_coords[0] + dims[0]):
                for y in range(start_coords[1], start_coords[1] + dims[1]):
                    if (x, y) in coords_used:
                        coords_overlap.add((x, y))
                    coords_used.add((x, y))

        for line in lines:
            group = p.findall(line)[0]
            elf_id = group[0]
            start_coords = [int(x) for x in group[1].split(',')]
            dims = [int(x) for x in group[2].split('x')]
            used = False
            for x in range(start_coords[0], start_coords[0] + dims[0]):
                for y in range(start_coords[1], start_coords[1] + dims[1]):
                    if (x, y) in coords_overlap:
                        used = True
                        break
                if used:
                    break
            if not used:
                return (len(coords_overlap), elf_id)
        
        return None


print('Part 1 & 2: ', part_1_2())