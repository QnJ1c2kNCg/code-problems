/// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that
/// i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
/// Notice that the solution set must not contain duplicate triplets.
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //Self::naive(&nums)
        //Self::hash_map(&nums)
        Self::binary_search(nums)
    }

    fn naive(nums: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut out: HashSet<Vec<i32>> = HashSet::default();
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut sorted = vec![nums[i], nums[j], nums[k]];
                        sorted.sort();
                        out.insert(sorted);
                    }
                }
            }
        }
        out.into_iter().collect()
    }

    fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
        let mut map = HashMap::with_capacity(nums.len());

        let mut out = Vec::default();

        for i in 0..nums.len() {
            let diff = target - nums[i];
            if let Some(index) = map.get(&diff) {
                out.push((nums[i], nums[*index as usize]));
            }
            map.insert(nums[i], i);
        }

        out
    }

    fn two_sum2(
        nums: &[i32],
        starting_index: usize,
        target: i32,
        map: &HashMap<i32, Vec<usize>>,
    ) -> Vec<Vec<i32>> {
        let mut out = Vec::default();

        for i in starting_index + 1..nums.len() {
            let diff = target - nums[i];
            if let Some(indexes) = map.get(&diff) {
                for index in indexes {
                    if *index != i && *index != starting_index {
                        let mut sorted = vec![nums[i], nums[starting_index], nums[*index]];
                        sorted.sort();
                        out.push(sorted);
                    }
                }
            }
        }

        out
    }

    fn hash_map(nums: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());
        let mut visited_targets: HashSet<i32> = HashSet::default();
        for (i, num) in nums.iter().enumerate() {
            map.entry(*num)
                .and_modify(|e| {
                    if e.len() < 3 {
                        e.push(i)
                    }
                })
                .or_insert(vec![i]);
        }

        let mut out: HashSet<Vec<i32>> = HashSet::default();
        for i in 0..nums.len() - 2 {
            if !visited_targets.contains(&nums[i]) {
                let founds = Self::two_sum2(&nums[..], i, -nums[i], &map);
                for found in founds {
                    out.insert(found);
                }
                visited_targets.insert(nums[i]);
            }
        }
        out.into_iter().collect()
    }

    fn binary_search(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut visited_targets: HashSet<i32> = HashSet::default();

        let mut out: HashSet<Vec<i32>> = HashSet::default();
        for i in 0..nums.len() - 2 {
            if !visited_targets.contains(&nums[i]) {
                let mut j = i + 1;
                let mut k = nums.len() - 1;

                while j < k {
                    let current_sum = nums[i] + nums[j] + nums[k];
                    if current_sum == 0 {
                        let mut sorted = vec![nums[i], nums[j], nums[k]];
                        sorted.sort();
                        out.insert(sorted);
                        // i decided to move j, but it doesn't matter
                        j += 1;
                    } else if current_sum > 0 {
                        // we move the right pointer towards the center
                        k -= 1;
                    } else {
                        // we move the left poitner towards the center
                        j += 1;
                    }
                }
            }
        }
        out.into_iter().collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        )
    }

    #[test]
    fn two() {
        let expected: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), expected)
    }

    #[test]
    fn three() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]])
    }
}
