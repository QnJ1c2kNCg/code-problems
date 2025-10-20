/// You have intercepted a secret message encoded as a string of numbers. The message is decoded via the following mapping:
/// "1" -> 'A'
/// "2" -> 'B'
/// ...
/// "25" -> 'Y'
/// "26" -> 'Z'
/// However, while decoding the message, you realize that there are many different ways you can decode the message because some codes are contained in other codes ("2" and "5" vs "25").
/// For example, "11106" can be decoded into:
/// "AAJF" with the grouping (1, 1, 10, 6)
/// "KJF" with the grouping (11, 10, 6)
/// The grouping (1, 11, 06) is invalid because "06" is not a valid code (only "6" is valid).
/// Note: there may be strings that are impossible to decode.
/// Given a string s containing only digits, return the number of ways to decode it. If the entire string cannot be decoded in any valid way, return 0.
/// The test cases are generated so that the answer fits in a 32-bit integer.

struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // Intuition:
        // Dynamic programming, kinda like the steps problem?
        let mut mem = std::collections::HashMap::default();
        Self::num_decodings_inner(&s.into_bytes(), &mut mem)
    }

    fn num_decodings_inner<'a>(
        s: &'a [u8],
        mem: &mut std::collections::HashMap<&'a [u8], i32>,
    ) -> i32 {
        if let Some(out) = mem.get(s) {
            return *out;
        }

        if s.len() == 0 {
            return 1;
        }

        let mut out = 0;
        // match the first char
        // ASCII code for number '1' to '9' inclusive
        if matches!(s[0], 49..=57) {
            out += Self::num_decodings_inner(&s[1..], mem);
        }
        // ASCII code for number '1'
        if s.len() >= 2 && s[0] == 49 {
            // ASCII code for number '0' to '9' inclusive
            if matches!(s[1], 48..=57) {
                out += Self::num_decodings_inner(&s[2..], mem);
            }
        }
        // ASCII code for number '2'
        if s.len() >= 2 && s[0] == 50 {
            // ASCII code for number '0' to '6' inclusive
            if matches!(s[1], 48..=54) {
                out += Self::num_decodings_inner(&s[2..], mem);
            }
        }

        mem.insert(s, out);

        out
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("06".to_owned()), 0);
        assert_eq!(
            Solution::num_decodings("111111111111111111111111111111111111111111111".to_owned()),
            1836311903
        );
    }
}
