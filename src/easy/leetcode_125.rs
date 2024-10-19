use std::ascii::AsciiExt;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut ss: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if (ss.len() == 0) {
            return true;
        }

        let mut l = 0 ;
        let mut r = ss.len() - 1;

        while l < r {
            if ss.get(l) != ss.get(r) {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}