use std::fs;
use std::collections::HashSet;

//fn create_preamble_set(series: &Vec<u64>, preamble_len: usize) -> HashSet<u64> {
fn create_preamble_set(series: &[u64]) -> HashSet<u64> {
    let mut preamble_set = HashSet::new();
    for i in 0..series.len() {
        for j in i + 1..series.len() {
            preamble_set.insert(series[i] + series[j]);
        }
    }
    preamble_set
}

fn part_1(contents: &str, preamble_len: usize) -> u64 {
    let series: Vec<u64> = contents.lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let mut preamble_idx = preamble_len;

    loop {
        let preamble_set = create_preamble_set(&series[preamble_idx - preamble_len..preamble_idx]);

        if preamble_set.contains(&series[preamble_idx]) {
            preamble_idx += 1;
        } else {
            return series[preamble_idx];
        }
    }
}

fn part_2(contents: &str, target: u64) -> u64 {
    let series: Vec<u64> = contents.lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    for i in 0..series.len() {
        let mut sum: u64 = series[i];
        for j in i + 1..series.len() {
            if sum == target {
                return *series[i..j].iter().min().unwrap() +
                       *series[i..j].iter().max().unwrap();
            }
            sum += series[j];
        }
    }


    0
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let p1 = part_1(&contents, 25);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", part_2(&contents, p1));
}
