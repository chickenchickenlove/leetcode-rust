struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 2]; n];

        dp[0][1] = nums[0];
        let mut ret = nums[0];

        if n > 1 {
            dp[1][0] = nums[0];
            dp[1][1] = nums[1];
            ret = ret.max(nums[1]);
        }

        for day in 2..n {
            dp[day][0] = dp[day - 1][1].max(dp[day - 1][0]);
            dp[day][1] = dp[day - 1][0] + nums[day];

            ret = ret.max(dp[day][0]).max(dp[day][1]);
        }
        ret
    }
}