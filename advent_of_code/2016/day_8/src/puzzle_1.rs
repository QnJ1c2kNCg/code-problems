// Day 8, puzzle 1 and 2
// http://adventofcode.com/2016/day/8

extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

fn rect(line: &str, screen: &mut [[char; 6]; 50]) {
    let re = Regex::new(r"\d+").unwrap();
    let x = re.captures(line).unwrap()[0].parse::<usize>().unwrap();
    let y = re.captures_iter(line).last().unwrap()[0].parse::<usize>().unwrap();

    for i in 0..x {
        for j in 0..y {
            screen[i][j] = '#';
        }
    }
}

fn rotate_row(line: &str, screen: &mut [[char; 6]; 50]) {
    let re = Regex::new(r"\d+").unwrap();
    let y = re.captures(line).unwrap()[0].parse::<usize>().unwrap();
    let amount = re.captures_iter(line).last().unwrap()[0].parse::<usize>().unwrap();

    let mut new_row: [char; 50] = ['.';50];

    for i in 0..50 {
        if i < amount {
            new_row[i] = screen[i + 50 - amount][y];
        } else {
            new_row[i] = screen[i - amount][y];
        }
    }

    for i in 0..50 {
        screen[i][y] = new_row[i];
    }
}

fn rotate_column(line: &str, screen: &mut [[char; 6]; 50]) {
    let re = Regex::new(r"\d+").unwrap();
    let x = re.captures(line).unwrap()[0].parse::<usize>().unwrap();
    let amount = re.captures_iter(line).last().unwrap()[0].parse::<usize>().unwrap();

    let mut new_column: [char; 6] = ['.';6];

    for j in 0..6 {
        if j < amount {
            new_column[j] = screen[x][j + 6 - amount];
        } else {
            new_column[j] = screen[x][j - amount];
        }
    }

    for j in 0..6 {
        screen[x][j] = new_column[j];
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut answer: [[char; 6]; 50] = [['.'; 6]; 50];

    for line in input.trim().split('\n') {
        if line.contains("rect") {
            rect(line, &mut answer);
        } else if line.contains("row") {
            rotate_row(line, &mut answer);
        } else {
            rotate_column(line, &mut answer);
        }
    }

    let mut puzzle_1_answer = 0;

    for j in 0..6 {
        for i in 0..50 {
            print!("{}", answer[i][j]);
            if answer[i][j] == '#' {
                puzzle_1_answer += 1;
            }
        }
        println!("");
    }
    println!("The number of lit pixels is {}." , puzzle_1_answer);
}
