// Day 2, puzzle 1
// http://adventofcode.com/2016/day/2

use std::io::prelude::*;
use std::fs::File;
use std::char;

fn main() {
    let mut file = File::open("puzzle_1_input.txt").unwrap();
    let mut  input = String::new();
    file.read_to_string(&mut input);

    let mut current_position = 5;

    let mut code = String::new();

    for value in input.chars() {
        match value {
            'U' => if current_position > 3      { current_position -= 3;},
            'D' => if current_position < 7      { current_position += 3;},
            'R' => if current_position % 3 != 0 { current_position += 1;},
            'L' => if current_position % 3 != 1 { current_position -= 1;},
            '\n'=> code.push(char::from_digit(current_position, 10).unwrap()),
             _ => println!("Wrong input"),
        };
    }
    println!("The code is {}.", code);
}
