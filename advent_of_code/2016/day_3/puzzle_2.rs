// Day 3, puzzle 2
// http://adventofcode.com/2016/day/3

use std::io::prelude::*;
use std::fs::File;

fn valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut triangles = Vec::<u32>::new();
    let mut nb_valid_triangle = 0;
    let mut current_triangle = 0;

    for line in input.trim().split('\n') {
        triangles.push(line[..5].trim().parse::<u32>().unwrap());
        triangles.push(line[5..10].trim().parse::<u32>().unwrap());
        triangles.push(line[10..].trim().parse::<u32>().unwrap());
    }

    for _ in 0..triangles.len() / 3 {
        if valid_triangle(triangles[current_triangle],
                          triangles[current_triangle + 3],
                          triangles[current_triangle + 6]) {
            nb_valid_triangle += 1;
        }
        current_triangle += 1;
        if current_triangle % 3 == 0 {
            current_triangle += 6;
        }
    }

    println!("The number of valid triangle is {}.", nb_valid_triangle);
}
