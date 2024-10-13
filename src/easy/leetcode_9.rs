pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let num_str = x.to_string();
        let chars: Vec<char> = num_str.chars().collect();

        if num_str.len() == 1 {
            return true
        };

        let end = (num_str.len() / 2) - 1;
        for i in 0..=end {
            if (chars[i] != chars[num_str.len()-1-i]) {
                return false;
            }
        }
        return true
    }
}