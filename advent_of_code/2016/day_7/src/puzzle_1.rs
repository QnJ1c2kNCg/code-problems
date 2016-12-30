// Day 7, puzzle 1
// http://adventofcode.com/2016/day/7

extern crate onig;

use onig::Regex;
use std::io::prelude::*;
use std::fs::File;

fn contains_abba(string: &str) -> bool {
    let mut passed_char = string.chars().nth(0).unwrap();
    for (index, ch) in string.char_indices() {
        let valid_index = index > 1 && index < string.len() - 1;
        if valid_index && ch == passed_char &&
            string.chars().nth(index + 1).unwrap() == string.chars().nth(index - 2).unwrap() &&
            string.chars().nth(index + 1).unwrap() != ch {
            return true;
        }
        passed_char = ch;
    }
    false
}

fn support_tls(line: &str) -> bool {
    let re_in_bracket = Regex::new(r"(?<=\[).+?(?=\])").unwrap();
    for in_bracket in re_in_bracket.captures_iter(line) {
        if contains_abba(in_bracket.at(0).unwrap()) {
            return false;
        }
    }
    if contains_abba(line) {
        return true;
    }
    false
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut nb_support_tls = 0;
    for line in input.trim().split('\n') {
        if support_tls(line) {
            nb_support_tls += 1;
        }
    }

    println!("The number of IP that supports TLS are : {}.", nb_support_tls);
}
