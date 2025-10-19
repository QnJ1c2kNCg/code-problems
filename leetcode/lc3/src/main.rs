/// Given a string s, find the length of the longest without duplicate characters.
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::with_capacity(26);
        let mut longest_substring = 0;
        let mut current_substring_start_index = 0;
        for (i, c) in s.into_bytes().into_iter().enumerate() {
            // Jotting thoughts...:
            // Try to insert in the hashmap
            // If the entry is vancant, then we insert and increment the longest chain
            // If the entry is present, then we have a duplicate. So we look at the
            // index that we found the previous char and start with + 1
            // If the entry is present, we also have to check if the previous sighting
            // was before or after the start of the substring.
            match map.entry(c) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    let previous_sighting = occupied_entry.get();
                    // the preivous sighting was before the current substring, we can ignore and
                    // simply update
                    if current_substring_start_index <= *previous_sighting {
                        // we start +1 from the previous sighting
                        current_substring_start_index = previous_sighting + 1;
                    }
                    occupied_entry.insert(i);
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert_entry(i);
                }
            }

            longest_substring =
                std::cmp::max(longest_substring, i - current_substring_start_index + 1);
        }

        longest_substring as i32
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
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
        assert_eq!(Solution::length_of_longest_substring("au".to_owned()), 2);
    }
}
