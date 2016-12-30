// Day 9, puzzle 1
// http://adventofcode.com/2016/day/9

extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut answer = String::new();
    let mut start_index = 0;

    loop {
        // Regex to find numbers
        let re = Regex::new(r"\(\d+").unwrap();
        let re1 = Regex::new(r"x\d+").unwrap();
        if !re.is_match(&input[start_index..]) {
            break;
        }
        let subsequent_char = re.captures_iter(&input[start_index..]).nth(0).unwrap()[0][1..].parse::<usize>().unwrap();
        let repeat = re1.captures(&input[start_index..]).unwrap()[0][1..].parse::<u32>().unwrap();

        let start_marker = re.find(&input[start_index..]).unwrap().0 + start_index; // index of the '('
        let end_marker = re1.find(&input[start_index..]).unwrap().1 + start_index;  // index of the ')'

        answer.push_str(&input[start_index..start_marker]);

        for _ in 0..repeat {
            answer.push_str(&input[end_marker + 1 .. end_marker + 1 + subsequent_char]);
        }
        start_index = end_marker + subsequent_char + 1;
    }

    answer.push_str(&input[start_index..]);

    println!("{}", answer.trim().len());
}
