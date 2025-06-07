use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut result: HashSet<Vec<i32>> = HashSet::new();

        for m in 1..sorted_nums.len() {
            let mut l = 0;
            let mut r = sorted_nums.len() - 1;

            while !(m <= l || r <= m) {
                let temp_sum = sorted_nums[l] + sorted_nums[m] + sorted_nums[r];
                if temp_sum == 0 {
                    result.insert(vec![sorted_nums[l], sorted_nums[m], sorted_nums[r]]);
                    l += 1;
                }
                else if temp_sum > 0  {
                    r -= 1;
                }
                else {
                    l += 1;
                }
            }
        }

        result
            .into_iter()
            .collect()
    }
}