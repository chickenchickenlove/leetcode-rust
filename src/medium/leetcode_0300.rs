
struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = vec![nums[0]];

        for &num in nums[1..].iter() {
            if num > sub[sub.len() - 1] {
                sub.push(num)
            }
            else {
                let mut i = 0;
                while num > sub[i] {
                    i += 1;
                }
                sub[i] = num;
            }
        }
        sub.len() as i32
    }
}