/// Given two integers left and right that represent the range [left, right],
/// return the bitwise AND of all numbers in this range, inclusive.

struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        // my intuition: whenever, we get a `0` in the out, we know that it won't flip
        // so we can move forward in the bits

        let mut out = left;
        let mut num = left;
        let mut bit_to_check = 0;
        while num <= right {
            out &= num;
            if out & 2i32.pow(bit_to_check) == 0 {
                // if the `bit_to_check` is set, we can skip ahead a lot
                bit_to_check += 1;
                if bit_to_check == 31 {
                    break;
                }
                num = std::cmp::max(2i32.pow(bit_to_check), num);
            } else {
                // kinda ugly to prevent overflow
                num = if let Some(num) = num.checked_add(2i32.pow(bit_to_check)) {
                    num
                } else {
                    break;
                };
            }
        }
        out & right
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
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(5, 8), 0);
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
        assert_eq!(Solution::range_bitwise_and(700000000, 2147483641), 0);
        assert_eq!(
            Solution::range_bitwise_and(2147483646, 2147483647),
            2147483646
        );
    }
}
