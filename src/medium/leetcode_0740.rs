use std::cmp::max;
use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut m: BTreeMap<i32, i32> = BTreeMap::new();
        for &n in nums.iter() {
            let t = n + m
                .get(&n)
                .unwrap_or(&0);
            m.insert(n, t);
        }

        let mut sorted_list: Vec<i32> = m
            .keys()
            .copied()
            .collect();

        let mut now_buy = vec![0; sorted_list.len()];
        now_buy[0] = *m.get(&sorted_list[0]).unwrap_or(&0);

        for (idx, &num) in sorted_list.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let idx32 = idx as i32;

            let idx_sum = *m.get(&num).unwrap_or(&0);

            let mut previous_buy = 0;
            if !m.contains_key(&(sorted_list[idx] - 1)) {
                previous_buy = max(previous_buy, now_buy[idx - 1] + idx_sum);
            }
            if m.contains_key(&(sorted_list[idx] - 1)) {
                previous_buy = *vec![previous_buy,
                                     now_buy[idx - 1],
                                     idx_sum]
                    .iter().max().unwrap();
            }
            if m.contains_key(&(sorted_list[idx] - 1)) && idx32 - 2 >= 0 {
                previous_buy = *vec![previous_buy,
                                     now_buy[idx - 2] + idx_sum]
                    .iter().max().unwrap();
            }

            if (idx32-3 >= 0) &&
                (sorted_list[idx-3] + 1 == sorted_list[idx-2]) &&
                (sorted_list[idx] - 1 == sorted_list[idx-1]) {
                previous_buy = max(previous_buy, now_buy[idx - 3] + idx_sum);
            }

            now_buy[idx] = previous_buy;
        }

        *now_buy
            .last()
            .unwrap()
    }
}