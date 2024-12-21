pub struct Solution;


use std::collections::HashSet;


fn is_end(left: usize, mid: usize, right: usize) -> bool {
    mid <= left || right <= mid
}

fn get_init_position(nums: &Vec<i32>) -> (usize, usize) {
    (0, nums.len() - 1)
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();

        let start = 1;
        let end = nums.len();

        let mut new_nums = nums.clone();

        new_nums.sort();

        for m in start..end {
            let (mut l, mut r) = get_init_position(&new_nums);
            while !is_end(l, m, r) {
                match new_nums[l] + new_nums[m] + new_nums[r] {
                    0 => {
                        result.insert((new_nums[l], new_nums[m], new_nums[r]));
                        l += 1;
                    }
                    x if x > 0 => r -= 1,
                    _ => l += 1,
                }
            }
        }

        // HashSet을 나중에 사용하지 않아도 되는 경우 (소유권이 이동되어 무효화 됨)
        // result.into_iter()
        //     .map(|(a, b, c)| vec![a, b, c])
        //     .collect();

        // HashSet을 나중에 사용해야하는 경우. (불변 참조자를 통해 데이터만 빌림)
        // result.iter()
        //     .map(|&(a, b, c)| vec![a, b, c])
        //     .collect();


        result
            .into_iter()
            .map(|(a, b, c)| vec![a, b, c])
            .collect()
    }
}