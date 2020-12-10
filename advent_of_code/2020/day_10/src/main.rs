use std::fs;

fn part_1(contents: &str) -> u64 {
    let mut input: Vec<u32> = Vec::new();
    for line in contents.lines() {
        input.push(line.parse::<u32>().unwrap());
    }
    input.sort();

    let mut current_adapter = 0;
    let mut n_1_jump = 0;
    let mut n_3_jump = 1;
    println!("{:?}", input);
    for a in input {
        let diff = a - current_adapter;
        current_adapter = a;
        match diff {
            1 => n_1_jump += 1,
            3 => n_3_jump += 1,
            _ => println!("Got of diff of: {}", diff),
        }
    }

    println!("Jump of size 1: {}", n_1_jump);
    println!("Jump of size 3: {}", n_3_jump);
    n_1_jump * n_3_jump
}

fn part_2(contents: &str) -> u64 {
    0
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
