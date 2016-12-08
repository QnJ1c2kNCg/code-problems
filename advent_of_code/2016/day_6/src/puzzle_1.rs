// Day 6, puzzle 1
// http://adventofcode.com/2016/day/6

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut answer = String::new();

    for i in 0..8 {
        let mut max_char = ' ';
        let mut max_count = 0;
        let mut char_count = vec![0;26];
        for line in input.trim().split('\n') {
            let current_char = line.chars().nth(i).unwrap();
            char_count[current_char as usize - 97] += 1; // 97 is the 'a' ASCII representation
            if char_count[current_char as usize - 97] >= max_count {
                max_count = char_count[current_char as usize - 97];
                max_char = current_char;
            }
        }
        answer.push(max_char);
    }

    println!("The answer is : {}", answer);
}
