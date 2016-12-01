// Day 1, puzzle 1
// http://adventofcode.com/2016/day/1

use std::io::prelude::*;
use std::fs::File;

enum Orientation { North, Est, South, West }

impl Orientation {
    fn rotate_right(&mut self) {
        *self = match self {
            &mut Orientation::North => Orientation::Est,
            &mut Orientation::Est   => Orientation::South,
            &mut Orientation::South => Orientation::West,
            &mut Orientation::West  => Orientation::North,
        };
    }

    fn rotate_left(&mut self) {
        *self = match self {
            &mut Orientation::North => Orientation::West,
            &mut Orientation::Est   => Orientation::North,
            &mut Orientation::South => Orientation::Est,
            &mut Orientation::West  => Orientation::South,
        };
    }

}

struct Coordinate { x: i32, y: i32 }

impl Coordinate {
    fn update(&mut self, distance: i32, orientation: &Orientation) {
        match orientation {
            &Orientation::North => self.y += distance,
            &Orientation::Est   => self.x += distance,
            &Orientation::South => self.y -= distance,
            &Orientation::West  => self.x -= distance,
        };
    }

    fn distance(self, coordinate: Coordinate) -> i32 {
        (self.x - coordinate.x).abs() + (self.y - coordinate.y).abs()
    }
}

fn find_destination(file_name: String) -> Coordinate {
    // Initial orientation and coordinate
    let mut orientation = Orientation::North;
    let mut coordinate = Coordinate { x: 0, y: 0 };

    // Opening the input file
    let mut file = File::open(file_name).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    // Splitting
    let values: Vec<&str> = input.split(", ").collect();

    for value in values {
        // Update the orientation
        match value.chars().nth(0).unwrap() {
            'R' => orientation.rotate_right(),
             _  => orientation.rotate_left(),
        };

        // Update the coordinate
        let distance: i32 = value[1..].trim().parse().unwrap();
        coordinate.update(distance, &orientation);
    }
    coordinate
}

fn main() {
    let final_coordinate = find_destination(String::from("puzzle_1_input.txt"));
    let distance = final_coordinate.distance(Coordinate { x: 0, y: 0 });
    println!("The distance is : {}" ,distance);
}
