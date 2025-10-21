/// There is an integer array nums sorted in ascending order (with distinct values).
/// Prior to being passed to your function, nums is possibly left rotated at an unknown index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be left rotated by 3 indices and become [4,5,6,7,0,1,2].

/// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

/// You must write an algorithm with O(log n) runtime complexity.
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Intuition: One of the half of the binary sort is fully sorted
        // XXX: Using `i32` instead of `usize` to deal with underflows
        let mut l = 0i32;
        let mut r = (nums.len() - 1) as i32;

        while l <= r {
            let pivot = l + ((r - l) / 2);
            if nums[pivot as usize] == target {
                return pivot as i32;
            }

            // Check if left half is sorted
            if nums[l as usize] <= nums[pivot as usize] {
                // left half is sorted
                if target < nums[pivot as usize] && target >= nums[l as usize] {
                    r = pivot - 1;
                } else {
                    l = pivot + 1;
                }
            } else {
                // left half is *not* sorted, means the right half is
                if target > nums[pivot as usize] && target <= nums[r as usize] {
                    l = pivot + 1;
                } else {
                    r = pivot - 1;
                }
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        // assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        // assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        // assert_eq!(Solution::search(vec![1], 0), -1);
        // assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
        // assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
        // assert_eq!(Solution::search(vec![1, 3], 3), 1);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }
}
