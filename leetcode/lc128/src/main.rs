/// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
/// You must write an algorithm that runs in O(n) time.
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.clone().into_iter().collect();

        let mut max_streak = 0;
        for num in &nums {
            if !set.contains(&(num - 1)) {
                // Here we know that it's the start of a streak, so we measure the length
                let mut current_num = *num;
                while set.contains(&current_num) {
                    current_num += 1;
                    max_streak = std::cmp::max(max_streak, current_num - num);
                }
            }
        }

        max_streak
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4)
    }

    #[test]
    fn two() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        )
    }

    #[test]
    fn three() {
        assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3)
    }

    #[test]
    fn four() {
        assert_eq!(
            Solution::longest_consecutive(vec![
                4, 2, 2, -4, 0, -2, 4, -3, -4, -4, -5, 1, 4, -9, 5, 0, 6, -8, -1, -3, 6, 5, -8, -1,
                -5, -1, 2, -9, 1
            ]),
            8
        )
    }
}
