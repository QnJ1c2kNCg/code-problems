use std::fs;
use std::collections::HashSet;

fn part_1(contents: &str) -> usize {
    let groups = contents.split("\n\n");

    let mut total: usize = 0;
    let mut unique_questions = HashSet::new();
    for group in groups {
        unique_questions.clear();
        // Remove whitespace
        let clean_group: String = group.split_whitespace().collect();
        for c in clean_group.chars() {
            unique_questions.insert(c);
        }
        total += unique_questions.len();
    }
    total
}

fn part_2(contents: &str) -> usize {
    let groups = contents.split("\n\n");

    let mut total: usize = 0;
    for group in groups {
        let mut unique_questions: HashSet<_> = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'].iter().cloned().collect();
        for person in group.split_whitespace() {
            let mut questions = HashSet::new();
            for question in person.chars() {
                questions.insert(question);
            }
            unique_questions.retain(|&v| questions.contains(&v));
        }

        total += unique_questions.len();
    }
    total
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
