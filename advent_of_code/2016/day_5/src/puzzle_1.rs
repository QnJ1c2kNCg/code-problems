// Day 5, puzzle 1
// http://adventofcode.com/2016/day/5
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "reyedfim";
    let mut answer = Vec::<char>::new();

    let mut index = 0;

    loop {
        let mut table = Md5::new();
        let input = format!("{}{}", input, &index.to_string());
        table.input_str(&input[..]);

        let result = table.result_str();
        if result.starts_with("00000") {
            answer.push(result.as_bytes()[5] as char);
            if answer.len() == 8 {
                break;
            }
        }
        index += 1;
    }

    println!("Answer : {:?}", answer);
}
