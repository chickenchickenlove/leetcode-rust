pub struct Solution;


use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            let want_value = target - num;
            if let Some(&value) = map.get(&want_value) {
                return vec![idx as i32, value]
            }
            map.insert(num, idx as i32);
        }
        return vec![0, 0];
    }
}




// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for i in 0..nums.len() - 1 {
//             for j in i+1..nums.len() {
//                 if nums[i] + nums[j] == target {
//                     return vec![i as i32, j as i32];
//                 }
//             }
//         }
//         return vec![0, 0];
//     }
// }









// Try 1
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut a = 0;
//         let mut b = 0;
//         for i in 0..nums.len() -1 {
//             for j in i + 1..nums.len() {
//                 if (nums[i] + nums[j] == target) {
//                     a = i;
//                     b = j;
//                     break;
//                 }
//             }
//         }
//         return vec![a as i32, b as i32];
//     }
// }
