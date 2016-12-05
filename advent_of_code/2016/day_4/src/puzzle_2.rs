// Day 4, puzzle 1
// http://adventofcode.com/2016/day/4
extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

fn real_room(name: &str, checksum: &str) -> bool {
    let mut mut_name = String::from(name);
    for i in checksum.chars() {
        let mut letter_count = (' ', 0usize);
        for character in "abcdefghijklmnopqrstuvwxyz".chars() {
            let current = (character, mut_name.matches(character).count() as usize);
            if current.1 > letter_count.1 {
                letter_count = current;
            }
        }
        if i != letter_count.0 {
            return false;
        }
        mut_name = mut_name.replace(letter_count.0, "-");
    }
    true
}

fn decrypt(name: &str, room_id: i32) -> String {
    let mut mut_name = String::with_capacity(name.len());
    for x in name.chars() {
        if x == '-' {
            mut_name.push(' ');
        } else {
            let mut letter = x as u8 - 'a' as u8;
            letter += (room_id % 26) as u8;
            letter = (letter % 26) + 'a' as u8;
            mut_name.push(letter as char);
        }
    }
    mut_name
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let re_name = Regex::new(r".*-").unwrap();
    let re_number = Regex::new(r"\d{1,100}").unwrap();
    let re_checksum = Regex::new(r"\[(.*?)\]").unwrap();

    for line in input.trim().split('\n') {
        let name = re_name.captures(line).unwrap();
        let number = re_number.captures(line).unwrap();
        let checksum = re_checksum.captures(line).unwrap();

        if real_room(&name[0], &checksum[1]) {
            let room_id = number[0].parse::<i32>().unwrap();
            if decrypt(&name[0], room_id).contains("northpole") {
                println!("The room ID is {}.", room_id);
            }
        }
    }
}
