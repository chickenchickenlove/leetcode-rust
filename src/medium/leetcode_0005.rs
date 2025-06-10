
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut mv = 1;
        let mut result = vec![0, 0];
        let my_char: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            dp[i][i] = true;
        }

        for i in 0..s.len()-1 {
            if my_char[i] == my_char[i+1] {
                dp[i][i+1] = true;
                mv = 2;
                result = vec![i as i32, i as i32 + 1];
            }
        }

        for diff in 2..s.len() {
            for i in 0..(s.len() - diff) {
                let j = i + diff;
                if my_char[i] == my_char[j] && dp[i+1][j-1] == true {
                    dp[i][j] = true;
                    result = vec![i as i32, j as i32];
                }
            }
        }

        let (a, b) = (result[0] as usize, result[1] as usize);
        s[a..=b].to_string()
    }
}