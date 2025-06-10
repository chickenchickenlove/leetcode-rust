
pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let d = 10_i32.pow(9) + 7;
        let mut dp = vec![0; (high + 1) as usize];

        dp[zero as usize] += 1;
        dp[one as usize] += 1;

        for i in 0..=high {
            if i - zero > 0 {
                dp[i as usize] = (dp[i as usize] + dp[(i-zero) as usize]) % d
            }
            if i - one > 0 {
                dp[i as usize] = (dp[i as usize] + dp[(i-one) as usize]) % d
            }
        };

        dp[low as usize..=high as usize]
            .iter()
            .fold(0, |acc, &y| (acc + y) % d) % d
    }
}