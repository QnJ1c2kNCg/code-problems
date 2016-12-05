// Day 1, puzzle 2
// http://adventofcode.com/2016/day/1

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

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

#[derive(Copy, Clone)]
struct Coordinate { x: i32, y: i32 }

impl Coordinate {
    fn update(&mut self, distance: i32,
              orientation: &Orientation,
              passed_coordinates: &mut Vec<Coordinate>) {
        match orientation {
            &Orientation::North => { for _ in 0..distance { self.y += 1; passed_coordinates.push(self.clone()); } },
            &Orientation::Est   => { for _ in 0..distance { self.x += 1; passed_coordinates.push(self.clone()); } },
            &Orientation::South => { for _ in 0..distance { self.y -= 1; passed_coordinates.push(self.clone()); } },
            &Orientation::West  => { for _ in 0..distance { self.x -= 1; passed_coordinates.push(self.clone()); } },
        };
    }

    fn distance(self, coordinate: Coordinate) -> i32 {
        (self.x - coordinate.x).abs() + (self.y - coordinate.y).abs()
    }
}

fn check_duplicate(passed_coordinates: &Vec<Coordinate>) -> Coordinate {
    let mut seen = HashSet::<(i32, i32)>::new();
    for coordinate in passed_coordinates {
        // Converting to tuple
        let tuple = (coordinate.x, coordinate.y);
        if !seen.insert(tuple) {
            return coordinate.clone();
        }
    }
    Coordinate {x: 0, y: 0}
}

fn find_destination(file_name: String) -> Coordinate {
    // Initial orientation and coordinate
    let mut orientation = Orientation::North;
    let mut coordinate = Coordinate { x: 0, y: 0 };
    let mut passed_coordinates = Vec::<Coordinate>::new();
    passed_coordinates.push(Coordinate { x: 0, y: 0 });

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
        coordinate.update(distance, &orientation, &mut passed_coordinates);
    }
    check_duplicate(&passed_coordinates)
}

fn main() {
    let final_coordinate = find_destination(String::from("puzzle_1_input.txt"));
    let distance = final_coordinate.distance(Coordinate { x: 0, y: 0 });
    println!("The distance is : {}" ,distance);
}
