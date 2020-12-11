use std::fs;
use std::collections::HashMap;
use std::convert::TryInto;

fn build_map(contents: &str) -> HashMap<i32, HashMap<i32, char>> {
    let mut map = HashMap::new();
    for (i, line) in contents.lines().enumerate() {
        map.insert(i as i32, HashMap::new());
        for (j, c) in line.chars().enumerate() {
            map.get_mut(&(i as i32)).unwrap().insert(j as i32, c);
        }
    }

    map
}

fn find_adjacent_seats(map: &HashMap<i32, HashMap<i32, char>>, i: &i32, j: &i32) -> Vec<char> {
    let mut adjacent_seats: Vec<char> = Vec::new();
    if map.contains_key(&(i - 1)) {
        adjacent_seats.push(*map.get(&(i - 1)).unwrap().get(&(j - 1)).unwrap_or(&'.'));
        adjacent_seats.push(*map.get(&(i - 1)).unwrap().get(&(j)).unwrap_or(&'.'));
        adjacent_seats.push(*map.get(&(i - 1)).unwrap().get(&(j + 1)).unwrap_or(&'.'));
    }
    if map.contains_key(&(i)) {
        adjacent_seats.push(*map.get(&(i)).unwrap().get(&(j - 1)).unwrap_or(&'.'));
        adjacent_seats.push(*map.get(&(i)).unwrap().get(&(j + 1)).unwrap_or(&'.'));
    }
    if map.contains_key(&(i + 1)) {
        adjacent_seats.push(*map.get(&(i + 1)).unwrap().get(&(j - 1)).unwrap_or(&'.'));
        adjacent_seats.push(*map.get(&(i + 1)).unwrap().get(&(j)).unwrap_or(&'.'));
        adjacent_seats.push(*map.get(&(i + 1)).unwrap().get(&(&(j + 1))).unwrap_or(&'.'));
    }
    adjacent_seats
}

fn transform(map: &mut HashMap<i32, HashMap<i32, char>>) {
    let ref_map = map.clone();
    for i in 0..map.len() {
        for j in 0..map.get(&(i as i32)).unwrap().len() {
            let adj_occupied_count = find_adjacent_seats(&ref_map, &(i as i32), &(j as i32)).iter().filter(|&s| *s == '#').count();

            if *ref_map.get(&(i as i32)).unwrap().get(&(j as i32)).unwrap() == 'L' {
                if  adj_occupied_count == 0 {
                    map.get_mut(&(i as i32)).unwrap().insert(j.try_into().unwrap(), '#');
                }
            } else if *ref_map.get(&(i as i32)).unwrap().get(&(j as i32)).unwrap() == '#' {
                if  adj_occupied_count >= 4 {
                    map.get_mut(&(i as i32)).unwrap().insert(j.try_into().unwrap(), 'L');
                }
            }
        }
    }
}

fn count_occupied_seats(map: & HashMap<i32, HashMap<i32, char>>) -> i32 {
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map.get(&(i as i32)).unwrap().len() {
            if *map.get(&(i as i32)).unwrap().get(&(j as i32)).unwrap() == '#' {
                count += 1;
            }
        }
    }

    count
}

fn part_1(contents: &str) -> i32 {
    let mut current_map = build_map(contents);
    let mut next_map = current_map.clone();

    loop {
        transform(&mut next_map);

        if current_map == next_map {
            break;
        }

        current_map = next_map.clone();
    }

    count_occupied_seats(&current_map)
}

fn find_diag_seats(map: &HashMap<i32, HashMap<i32, char>>, i: &i32, j: &i32) -> Vec<char> {
    let mut diag_seats: Vec<char> = Vec::new();

    let mut iter = 1;
    // Top left
    loop {
        if map.contains_key(&(i - iter)) {
            if map.get(&(i - iter)).unwrap().contains_key(&(j - iter)) {
                let c = map.get(&(i - iter)).unwrap().get(&(j - iter)).unwrap();
                if *c != '.' {
                    diag_seats.push(*c);
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Top
    loop {
        if map.contains_key(&(i - iter)) {
            let c = map.get(&(i - iter)).unwrap().get(&(j)).unwrap();
            if *c != '.' {
                diag_seats.push(*c);
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Bot
    loop {
        if map.contains_key(&(i + iter)) {
            let c = map.get(&(i + iter)).unwrap().get(&(j)).unwrap();
            if *c != '.' {
                diag_seats.push(*c);
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Top right
    loop {
        if map.contains_key(&(i - iter)) {
            if map.get(&(i - iter)).unwrap().contains_key(&(j + iter)) {
                let c = map.get(&(i - iter)).unwrap().get(&(j + iter)).unwrap();
                if *c != '.' {
                    diag_seats.push(*c);
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Right
    loop {
        if map.get(&(i)).unwrap().contains_key(&(j + iter)) {
            let c = map.get(&(i)).unwrap().get(&(j + iter)).unwrap();
            if *c != '.' {
                diag_seats.push(*c);
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Left
    loop {
        if map.get(&(i)).unwrap().contains_key(&(j - iter)) {
            let c = map.get(&(i)).unwrap().get(&(j - iter)).unwrap();
            if *c != '.' {
                diag_seats.push(*c);
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Bot right
    loop {
        if map.contains_key(&(i + iter)) {
            if map.get(&(i + iter)).unwrap().contains_key(&(j + iter)) {
                let c = map.get(&(i + iter)).unwrap().get(&(j + iter)).unwrap();
                if *c != '.' {
                    diag_seats.push(*c);
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter = 1;
    // Bot left
    loop {
        if map.contains_key(&(i + iter)) {
            if map.get(&(i + iter)).unwrap().contains_key(&(j - iter)) {
                let c = map.get(&(i + iter)).unwrap().get(&(j - iter)).unwrap();
                if *c != '.' {
                    diag_seats.push(*c);
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        iter += 1;
    }

    diag_seats
}

fn transform2(map: &mut HashMap<i32, HashMap<i32, char>>) {
    let ref_map = map.clone();
    for i in 0..map.len() {
        for j in 0..map.get(&(i as i32)).unwrap().len() {
            let diag_occupied_count = find_diag_seats(&ref_map, &(i as i32), &(j as i32)).iter().filter(|&s| *s == '#').count();

            if *ref_map.get(&(i as i32)).unwrap().get(&(j as i32)).unwrap() == 'L' {
                if  diag_occupied_count == 0 {
                    map.get_mut(&(i as i32)).unwrap().insert(j.try_into().unwrap(), '#');
                }
            } else if *ref_map.get(&(i as i32)).unwrap().get(&(j as i32)).unwrap() == '#' {
                if  diag_occupied_count >= 5 {
                    map.get_mut(&(i as i32)).unwrap().insert(j.try_into().unwrap(), 'L');
                }
            }
        }
    }
}

fn part_2(contents: &str) -> i32 {
    let mut current_map = build_map(contents);
    let mut next_map = current_map.clone();

    loop {
        transform2(&mut next_map);

        if current_map == next_map {
            break;
        }

        current_map = next_map.clone();
    }

    count_occupied_seats(&current_map)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
