struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 2]; n];
        let mut dpn = vec![vec![0; 2]; n];

        dp[0][1] = nums[0];
        let mut ret = nums[0];
        if n > 1 {
            dp[1][0] = nums[0];
            dp[1][1] = nums[1];
            dpn[1][1] = nums[1];
            ret = ret.max(nums[1]);
        }

        for day in 2..n {
            dp[day][1] = if n - 1 == day { dp[day - 1][0] } else { dp[day - 1][0] + nums[day] };

            dp[day][0] = dp[day-1][1].max(dp[day-1][0]);
            dpn[day][0] = dpn[day-1][1].max(dpn[day-1][0]);
            dpn[day][1] = dpn[day-1][0] + nums[day];

            if let Some(&v) = dp[day].iter().max() {
                ret = ret.max(v);
            }

            if let Some(&v) = dpn[day].iter().max() {
                ret = ret.max(v);
            }
        }
        ret
    }
}