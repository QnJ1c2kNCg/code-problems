from datetime import datetime, timedelta
import re

def daterange(start_date, end_date):
    for n in range(int ((end_date - start_date).minutes)):
        yield start_date + timedelta(n)

def part_1_2():
    lines = list()
    with open('input.txt', 'r') as f:
        lines = f.read().splitlines()

    # Sort in chronological order
    datetime_re = re.compile('(\d+-\d+-\d+ \d+:\d+)')
    lines.sort(key=lambda x: datetime.strptime(datetime_re.findall(x)[0], '%Y-%m-%d %H:%M'))

    # Have a map of guards
    # Key -> guard ID
    # Value -> Dict<Minute, Occurence>
    guards = dict()
    current_guard_id = 0
    guard_id_re = re.compile('#(\d+)')
    start_sleep = None
    for line in lines:
        if 'begins shift' in line:
            current_guard_id = guard_id_re.findall(line)[0]
        elif 'falls asleep' in line:
            start_sleep = datetime.strptime(datetime_re.findall(line)[0], '%Y-%m-%d %H:%M')
        elif 'wakes up' in line:
            end_sleep = datetime.strptime(datetime_re.findall(line)[0], '%Y-%m-%d %H:%M')
            delta = timedelta(minutes=1)

            current_guard_map = None
            if current_guard_id not in guards:
                current_guard_map = dict()
                guards[current_guard_id]  = current_guard_map
            else:
                current_guard_map = guards[current_guard_id]

            current_time = start_sleep
            while current_time < end_sleep:
                hours = current_time.time()
                current_guard_map[hours] = current_guard_map.get(hours, 0) + 1
                current_time += delta


    # Find the guard ID with the biggest dictionary & multiply with the biggest occurence
    max_id = max(guards, key=lambda k: sum(guards[k].values()))
    max_guard_map = guards[max_id]
    max_occurence = int(max(max_guard_map, key=lambda k: max_guard_map[k]).strftime('%M'))

    max_id_2 = max(guards, key=lambda k: max(guards[k].values()))
    max_guard_map_2 = guards[max_id_2]
    max_occurence_2 = int(max(max_guard_map_2, key=lambda k: max_guard_map_2[k]).strftime('%M'))
    return (int(max_id) * max_occurence, int(max_id_2) * max_occurence_2)


print('Part 1 & 2: ', part_1_2())