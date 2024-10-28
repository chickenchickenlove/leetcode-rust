struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return nums[0];
        }

        let mut max_value = 0;
        let mut min_value = 0;
        let mut result = 0;

        for i in 0..n {
            let current = nums[i];
            let temp_max = current
                .max(max_value * current)
                .max(min_value * current);

            min_value = current
                .min(min_value * current)
                .min(max_value * current);

            result = result.max(temp_max);
            max_value = temp_max;
        }
        result
    }
}