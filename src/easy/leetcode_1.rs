pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a = 0;
        let mut b = 0;
        for i in 0..nums.len() -1 {
            for j in i + 1..nums.len() {
                if (nums[i] + nums[j] == target) {
                    a = i;
                    b = j;
                    break;
                }
            }
        }
        return vec![a as i32, b as i32];
    }
}
