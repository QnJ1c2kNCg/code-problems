use std::fs;
use regex::Regex;

fn part_1(contents: &str) -> Vec<&str> {
    let passports = contents.split("\n\n");
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports: Vec<&str> = Vec::new();
    'outer: for passport in passports {
        for field in &fields {
            if !passport.contains(field) {
                continue 'outer;
            }
        }
        valid_passports.push(passport);
    }

    valid_passports
}

fn part_2(passports: &Vec<&str>) -> i32 {
    let re_byr = Regex::new(r"byr:(\d{4})").unwrap();
    let re_iyr = Regex::new(r"iyr:(\d{4})").unwrap();
    let re_eyr = Regex::new(r"eyr:(\d{4})").unwrap();
    let re_hgt = Regex::new(r"hgt:(?P<hgt>\d+)(in|cm)").unwrap();
    let re_hcl = Regex::new(r"hcl:#(?P<hcl>[a-f0-9]{6})").unwrap();
    let re_ecl = Regex::new(r"ecl:(?P<ecl>(amb|blu|brn|gry|grn|hzl|oth){1})").unwrap();
    let re_pid = Regex::new(r"pid:(\d{9})(\s|$)").unwrap();
    let mut valid_passports = 0;

    for passport in passports {
        match re_byr.captures(passport) {
            Some(caps) => {
                let byr_value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                if byr_value < 1920 || byr_value > 2002 {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        match re_iyr.captures(passport) {
            Some(caps) => {
                let iyr_value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                if iyr_value < 2010 || iyr_value > 2020 {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        match re_eyr.captures(passport) {
            Some(caps) => {
                let eyr_value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                if eyr_value < 2020 || eyr_value > 2030 {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        match re_hgt.captures(passport) {
            Some(caps) => {
                let hgt_value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let hgt_unit = caps.get(2).unwrap().as_str();
                if hgt_unit == "cm" && (hgt_value < 150 || hgt_value > 193) {
                    continue;
                }
                if hgt_unit == "in" && (hgt_value < 59 || hgt_value > 76) {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        match re_hcl.captures(passport) {
            Some(_) => {
            }
            None => {
                continue;
            }
        }
        match re_ecl.captures(passport) {
            Some(_) => {
            }
            None => {
                continue;
            }
        }
        match re_pid.captures(passport) {
            Some(_) => {
            }
            None => {
                continue;
            }
        }

        valid_passports += 1;
    }

    valid_passports
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let valid_passports = part_1(&contents);

    println!("Part 1: {}", valid_passports.len());
    println!("Part 2: {}", part_2(&valid_passports));
}
