use std::fs;
use std::collections::HashMap;

fn part_1(contents: &str) -> u64 {
    let mut input: Vec<u32> = Vec::new();
    for line in contents.lines() {
        input.push(line.parse::<u32>().unwrap());
    }
    input.sort();

    let mut current_adapter = 0;
    let mut n_1_jump = 0;
    let mut n_3_jump = 1;
    for a in input {
        let diff = a - current_adapter;
        current_adapter = a;
        match diff {
            1 => n_1_jump += 1,
            3 => n_3_jump += 1,
            _ => println!("Got of diff of: {}", diff),
        }
    }

    n_1_jump * n_3_jump
}

fn count_permutation(input: &Vec<u64>, current_idx: usize, memoization: &mut HashMap<usize, u64>) -> u64 {
    if memoization.contains_key(&current_idx) {
        return *memoization.get(&current_idx).unwrap();
    }
    if current_idx == input.len() - 1 {
        return 1;
    }

    let mut permutation_count = 0;
    for i in current_idx + 1..current_idx + 4 {
        if i < input.len() && input[i] - input[current_idx] <= 3 {
            permutation_count += count_permutation(&input, i, memoization);
        }
    }

    memoization.insert(current_idx, permutation_count);
    *memoization.get(&current_idx).unwrap()
}

fn part_2(contents: &str) -> u64 {
    let mut input: Vec<u64> = Vec::new();
    for line in contents.lines() {
        input.push(line.parse::<u64>().unwrap());
    }
    input.sort();
    input.insert(0, 0);

    let mut memoization = HashMap::new();
    memoization.insert(input.len() - 1, 1);

    count_permutation(&input, 0, &mut memoization)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
