use std::fs;
use regex::Regex;

fn part_1(contents: &str) -> i32 {
    let mut valid_passwords = 0;
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<password>\w+)").unwrap();
    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.name("letter").unwrap().as_str();
        let password = caps.name("password").unwrap().as_str();

        let occurences = password.matches(letter).count();
        if min <= occurences && occurences <= max {
            valid_passwords = valid_passwords + 1;
        }
    }

    return valid_passwords;
}

fn part_2(contents: &str) -> i32 {
    let mut valid_passwords = 0;
    let re = Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<letter>.): (?P<password>\w+)").unwrap();
    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let first = caps.name("first").unwrap().as_str().parse::<usize>().unwrap();
        let second = caps.name("second").unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.name("letter").unwrap().as_str().chars().nth(0).unwrap();
        let password: Vec<char> = caps.name("password").unwrap().as_str().chars().collect();

        if (password[first - 1] == letter && password[second - 1] != letter) ||
           (password[first - 1] != letter && password[second - 1] == letter) {
            valid_passwords = valid_passwords + 1;
        }
    }

    return valid_passwords;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
