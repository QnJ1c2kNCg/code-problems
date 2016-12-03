// Day 3, puzzle 1
// http://adventofcode.com/2016/day/3

use std::io::prelude::*;
use std::fs::File;

fn valid_triangle(triangle: &str) -> bool {
    let a = triangle[..5].trim().parse::<u32>().unwrap();
    let b = triangle[5..10].trim().parse::<u32>().unwrap();
    let c = triangle[10..].trim().parse::<u32>().unwrap();

    a + b > c && a + c > b && b + c > a
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let triangles: Vec<&str> = input.trim().split('\n').collect();
    let mut nb_valid_triangle = 0;

    for triangle in triangles {
        if valid_triangle(triangle) {
            nb_valid_triangle += 1;
        }
    }

    println!("The number of valid triangle is {}.", nb_valid_triangle);
}
