use std::cmp;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring;

    #[test]
    fn test_abc() {
        assert_eq!(3, length_of_longest_substring(String::from("abcabcbb")))
    }
    #[test]
    fn test_bbb() {
        assert_eq!(1, length_of_longest_substring(String::from("bbbbbbbb")))
    }
    #[test]
    fn test_wke() {
        assert_eq!(3, length_of_longest_substring(String::from("pwwkew")))
    }
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen = HashMap::new();
    let mut last_start = -1;
    let mut result = 0;

    for (i, c) in s.chars().enumerate() {
        let entry = last_seen.entry(c).or_insert(-2);
        if (*entry).ge(&last_start) {
            last_start = *entry
        }

        result = cmp::max(i as i32 - last_start, result);

        *entry = i as i32
    }

    result
}
