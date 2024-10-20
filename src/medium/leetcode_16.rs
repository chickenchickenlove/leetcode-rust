pub struct Solution;


impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut new_nums = nums.clone();
        new_nums.sort();

        let mut min_diff = 2147483647;
        let mut result = 0;

        for mi in 0..new_nums.len() - 1 {
            let (mut l, mut r) = (mi + 1, new_nums.len() - 1);
            while l < r {
                let t = new_nums[l] + new_nums[mi] + new_nums[r];
                let diff = (target - t).abs();

                if diff < min_diff {
                    min_diff = diff;
                    result = t;
                }

                match t {
                    x if x == target => return t,
                    x if x < target => l += 1,
                    _ => r-=1,
                }
            }
        }
        result
    }
}