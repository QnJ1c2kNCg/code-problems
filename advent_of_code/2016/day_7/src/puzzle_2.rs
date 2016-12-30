// Day 7, puzzle 2
// http://adventofcode.com/2016/day/7

extern crate onig;

use onig::Regex;
use std::io::prelude::*;
use std::fs::File;

fn contains_abba(string: &str, aba: &mut Vec<String>){
    let mut passed_passed_char = string.chars().nth(0).unwrap();
    let mut passed_char        = string.chars().nth(1).unwrap();

    for (index, ch) in string.char_indices() {
        if  index > 1 && ch == passed_passed_char && ch != passed_char {
           let mut add = String::new();
           add.push(passed_char);
           add.push(ch);
           add.push(passed_char);
           aba.push(add);
        }
        passed_passed_char = passed_char;
        passed_char = ch;
    }
}

fn support_ssl(line: &str) -> bool {
    let re_in_bracket = Regex::new(r"(?<=\[).+?(?=\])").unwrap();
    let mut aba: Vec<String> = Vec::new();

    for in_bracket in re_in_bracket.captures_iter(line) {
        contains_abba(in_bracket.at(0).unwrap(), &mut aba);
    }

    let replaced_copy: String = re_in_bracket.replace_all(line, "");
    for x in aba {
        if replaced_copy.contains(&x) {
            return true;
        }
    }
    false
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut nb_support_ssl = 0;
    for line in input.trim().split('\n') {
        if support_ssl(line) {
            nb_support_ssl += 1;
        }
    }

    println!("The number of IP that supports TLS are : {}.", nb_support_ssl);
}
