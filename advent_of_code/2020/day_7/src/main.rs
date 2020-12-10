use std::fs;
use regex::Regex;
use std::collections::HashMap;
use multimap::MultiMap;


fn contains_bag(bags: &MultiMap<&str, &str>, starting_bag: &str, target_bag: &str) -> bool {

    // The "no other" case
    if !bags.contains_key(starting_bag) {
        return false;
    }

    for bag in bags.get_vec(starting_bag).unwrap() {
        // The matching case
        if bag == &target_bag {
            return true;
        }
        if contains_bag(bags, bag, target_bag) {
            return true;
        }
    }

    false
}

fn part_1(contents: &str, target_bag: &str) -> usize {
    let re = Regex::new(r"((?P<bag>\w+ \w+)) bag").unwrap();

    let mut bags = MultiMap::new();

    for line in contents.lines() {
        let captures: Vec<regex::Captures<'_>> = re.captures_iter(line).collect();
        for i in 1..captures.len() {
            bags.insert(captures[0].name("bag").unwrap().as_str(), captures[i].name("bag").unwrap().as_str());
        }
    }

    let mut answer: usize = 0;
    for (key, _) in bags.iter() {
        if contains_bag(&bags, key, target_bag) {
            answer += 1;
        }
    }

    answer
}

fn count_bags(bags: &MultiMap<&str, &str>, bags_count: &HashMap<String, i32>, starting_bag: &str) -> i32 {
    let mut count = 0;
    if !bags.contains_key(starting_bag) {
        return 0;
    }
    for bag in bags.get_vec(starting_bag).unwrap() {
        count += bags_count.get(&(starting_bag.to_string() + &bag.to_string())).unwrap() * (1 + count_bags(bags, bags_count, bag));
    }
    count
}

fn part_2(contents: &str, target_bag: &str) -> i32 {
    let re_names = Regex::new(r"((?P<bag>\w+ \w+)) bag").unwrap();
    let re_count = Regex::new(r" (?P<count>\d+) ").unwrap();

    let mut bags = MultiMap::new();
    let mut bags_count: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let captures_names: Vec<regex::Captures<'_>> = re_names.captures_iter(line).collect();
        let captures_count: Vec<regex::Captures<'_>> = re_count.captures_iter(line).collect();

        for i in 1..captures_names.len() {
            if captures_count.len() == 0 {
                continue;
            }
            bags.insert(captures_names[0].name("bag").unwrap().as_str(), captures_names[i].name("bag").unwrap().as_str());
            bags_count.insert(captures_names[0].name("bag").unwrap().as_str().to_string() +
                              &captures_names[i].name("bag").unwrap().as_str().to_string(),
                              captures_count[i - 1].name("count").unwrap().as_str().parse::<i32>().unwrap());
        }
    }

    count_bags(&bags, &bags_count, &target_bag)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents, "shiny gold"));
    println!("Part 2: {}", part_2(&contents, "shiny gold"));
}
