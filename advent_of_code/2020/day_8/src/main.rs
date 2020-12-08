use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryInto;

fn execute(lines: Vec<&str>) -> (bool, i32) {
    let re = Regex::new(r"^(?P<op>\w+) (?P<offset>(\+|-)\d+)$").unwrap();
    let mut idx: usize = 0;
    let mut acc: i32 = 0;
    let mut visited_idx = HashSet::new();
    loop {
        if visited_idx.contains(&idx) {
            return (false, acc);
        }

        if idx == lines.len() {
            return (true, acc);
        }
        visited_idx.insert(idx);
        let line = lines[idx];
        let caps = re.captures(line).unwrap();
        let op = caps.name("op").unwrap().as_str();
        let offset = caps.name("offset").unwrap().as_str().parse::<i32>().unwrap();

        match op {
            "nop" => idx += 1,
            "acc" => {
                acc += offset;
                idx += 1;
            },
            "jmp" => idx = (idx as i32 + offset).try_into().unwrap(),
            _ => println!("ERROR!"),
        }
    }
}

fn part_1(contents: &str) -> i32 {
    let lines: Vec<&str> = contents.lines().collect();

    let (_, acc) = execute(lines);

    acc
}

fn part_2(contents: &str) -> i32 {
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_owned()).collect();

    for i in 0..lines.len() {
        // Change the `jmp`s and `nop`s
        lines[i] = if lines[i].contains("jmp") {
            lines[i].replace("jmp", "nop")
        } else {
            lines[i].replace("nop", "jmp")
        };

        let (terminated, acc) = execute(lines.iter().map(|s| s as &str).collect());
        if terminated {
            return acc;
        }


        // Change the `jmp`s and `nop`s
        lines[i] = if lines[i].contains("jmp") {
            lines[i].replace("jmp", "nop")
        } else {
            lines[i].replace("nop", "jmp")
        };
    }
    0
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
