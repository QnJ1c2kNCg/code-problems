use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("AoC Day 1");
    let file = File::open("inputs/day_1").unwrap();
    let reader = BufReader::new(file);

    let depths: Vec<u32> = reader
        .lines()
        .filter_map(|l| l.unwrap().parse::<u32>().ok())
        .collect();

    // wanted to try using iterators
    let count_increases = |vec: &Vec<u32>| {
        vec.iter()
            .zip(vec.iter().skip(1))
            .filter(|(x, y)| y > x)
            .count()
    };

    println!("Part 1: {}", count_increases(&depths));

    let mut summed_depths = Vec::with_capacity(depths.len());
    for i in 0..depths.len() - 2 {
        summed_depths.push(depths[i] + depths[i + 1] + depths[i + 2]);
    }

    println!("Part 2: {}", count_increases(&summed_depths));
}
