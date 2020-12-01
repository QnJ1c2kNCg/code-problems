use std::fs;

fn part_1(contents: &str) -> i32 {
    for a in contents.lines() {
        for b in contents.lines() {
            let x = a.parse::<i32>().unwrap();
            let y = b.parse::<i32>().unwrap();
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    return 0;
}

fn part_2(contents: &str) -> i32 {
    for a in contents.lines() {
        for b in contents.lines() {
            for c in contents.lines() {
                let x = a.parse::<i32>().unwrap();
                let y = b.parse::<i32>().unwrap();
                let z = c.parse::<i32>().unwrap();
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    return 0;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
