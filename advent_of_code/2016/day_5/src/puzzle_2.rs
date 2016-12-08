// Day 5, puzzle 2
// http://adventofcode.com/2016/day/5
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "reyedfim";
    let mut answer = vec![' ';8];

    let mut index = 0;

    loop {
        let mut table = Md5::new();
        let input = format!("{}{}", input, &index.to_string());
        table.input_str(&input[..]);

        let result = table.result_str();
        // If the sixth number is between 0 and 7 (in 47 and 56 in ASCII)
        if result.starts_with("00000") && result.as_bytes()[5] > 47 && result.as_bytes()[5] < 56 {
            let i = result.chars().nth(5).unwrap().to_digit(10).unwrap() as usize;
            if answer[i] == ' ' {
                answer[i] = result.as_bytes()[6] as char;
                if !answer.contains(&' ') {
                    break;
                }
            }
        }
        index += 1;
    }

    println!("Answer : {:?}", answer);
}
