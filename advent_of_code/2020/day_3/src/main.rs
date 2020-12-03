use std::fs;

fn build_map(contents: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    // This could probably be more idiomatic
    for line in contents.lines() {
        let row = line.chars().collect::<Vec<_>>();
        map.push(row);
    }

    map
}

fn count_trees(map: &Vec<Vec<char>>, slopes: &Vec<(usize, usize)>) -> u64 {
    let mut total_trees: u64 = 1;

    for slope in slopes {
        let mut current_pos = (0, 0); // (x, y)
        let mut n_trees = 0;
        loop {
            // Move current pos
            current_pos.0 += slope.0;
            current_pos.1 += slope.1;

            // Check if we've reached the end
            if current_pos.1 >= map.len() {
                total_trees *= n_trees;
                break;
            }

            // Otherwise, check if it's a tree
            if map[current_pos.1][current_pos.0 % 31] == '#' {
                n_trees += 1;
            }
        }
    }

    total_trees
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let map = build_map(&contents);

    let slope_1 = vec![(3, 1)];
    let slope_2 = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    println!("Part 1: {}", count_trees(&map, &slope_1));
    println!("Part 2: {}", count_trees(&map, &slope_2));
}
