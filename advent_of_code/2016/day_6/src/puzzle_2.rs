// Day 6, puzzle 2
// http://adventofcode.com/2016/day/6

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut answer = String::new();

    for i in 0..8 {
        let mut min_count = i32::max_value();
        let mut min_index = 0;
        let mut char_count = vec![0;26];

        for line in input.trim().split('\n') {
            let current_char = line.chars().nth(i).unwrap();
            char_count[current_char as usize - 97] += 1; // 97 is the 'a' ASCII representation
        }

        for (index, count) in char_count.iter().enumerate() {
            if count < &min_count {
                min_count = *count;
                min_index = index;
            }
        }
        answer.push((min_index + 97) as u8 as char);
    }

    println!("The answer is : {}", answer);
}
