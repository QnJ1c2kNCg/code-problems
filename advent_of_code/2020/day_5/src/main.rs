use std::fs;

fn interpret(line: &str, mut offset: u32) -> u32 {
    let mut start  = 0;
    for c in line.chars() {
        offset /= 2;
        if c == 'B' || c == 'R' {
            start += offset + 1;
        }
    }

    start + offset
}

fn part_1(contents: &str) -> u32 {
    let mut max = 0;
    for line in contents.lines() {
        let row = interpret(&line[0..7], 127);
        let col = interpret(&line[7..], 7);
        let id = (row * 8) + col;
        if id > max {
            max = id;
        }
    }
    max
}

fn part_2(contents: &str) -> u32 {
    let mut ids: Vec<u32> = Vec::new();
    for line in contents.lines() {
        let row = interpret(&line[0..7], 127);
        let col = interpret(&line[7..], 7);
        let id = (row * 8) + col;
        ids.push(id);
    }
    ids.sort();
    for (idx, id) in ids.iter().enumerate() {
        let next_id: u32 = ids[idx + 1];
        if *id != next_id - 1 {
            return id + 1;
        }

    }
    0
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
