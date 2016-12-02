// Day 2, puzzle 2
// http://adventofcode.com/2016/day/2

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("puzzle_1_input.txt").unwrap();
    let mut  input = String::new();
    file.read_to_string(&mut input);

    let mut current_index = 10; // 10 is the index of the number 5

    let keypad = vec!['0', '0', '1', '0', '0',
                      '0', '2', '3', '4', '0',
                      '5', '6', '7', '8', '9',
                      '0', 'A', 'B', 'C', '0',
                      '0', '0', 'D', '0', '0'];

    let mut code = String::new();

    for value in input.chars() {
        match value {
            'U' => if current_index > 4      && keypad[current_index - 5] != '0' { current_index -= 5;},
            'D' => if current_index < 20     && keypad[current_index + 5] != '0' { current_index += 5;},
            'R' => if current_index % 4 != 0 && keypad[current_index + 1] != '0' { current_index += 1;},
            'L' => if current_index % 5 != 0 && keypad[current_index - 1] != '0' { current_index -= 1;},
            '\n'=> code.push(keypad[current_index]),
             _ => println!("Wrong input"),
        };
    }
    println!("The code is {}.", code);
}
