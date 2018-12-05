def part_1(content=None):
    content = content
    if not content:
        with open('input.txt', 'r') as f:
            content = f.read()

    has_changed = True
    while has_changed:
        has_changed = False
        for i in range(0, len(content) - 1):
            if content[i].islower() and content[i + 1] == content[i].upper() or \
               content[i].isupper() and content [i + 1] == content[i].lower():
                content = content[:i] + content[(i + 2):]
                has_changed = True
                break
    return content

def part_2(content=None):
    if not content:
        content = part_1()
    letters = set(content.lower())

    minimum = 1000000
    current_min = 1000000
    for c in letters:
        new_content = content.replace(c, '')
        new_content = new_content.replace(c.upper(), '')
        current_min = len(part_1(new_content))
        if current_min < minimum:
            minimum = current_min
    return minimum



part_1_str = part_1()
print('Part 1: ', len(part_1_str))
print('Part 2: ', part_2(part_1_str))