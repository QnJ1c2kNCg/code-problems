// Day 9, puzzle 2
// http://adventofcode.com/2016/day/9

use std::io::prelude::*;
use std::fs::File;

// Need a lot a cleaning... the borrowing...
fn decompress(s: &str) -> usize {
    let mut lenght: usize = 0;

    let mut s = String::from(s);

    // Exit condition
    if !s.contains('(') {
        return s.len();
    }

    while s.contains('(') {
        // Add the character before the first '('
        lenght += s.find('(').unwrap();
        // s becommes the end of the string starting with '('
        s = s[s.find('(').unwrap()..].to_string();

        let s1 = s.clone();
        let data: Vec<&str> = s1[1..s.find(')').unwrap()].split('x').collect();

        s = s[s.find(')').unwrap() + 1..].to_string();

        // Recursive calls
        lenght += decompress(&s[..data[0].parse::<usize>().unwrap()]) * data[1].parse::<usize>().unwrap();

        s = s[data[0].parse::<usize>().unwrap()..].to_string();
    }
    lenght += s.len();
    lenght
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let answer = decompress(&mut input);
    println!("{}", answer - 1); // -1 for the EOF whitespace
}
