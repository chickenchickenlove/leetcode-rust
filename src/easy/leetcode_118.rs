use std::mem::size_of;

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![1]];

        for i in 1..num_rows {
            let mut r = vec![1];
            let previous_row: &Vec<i32> = &ans[(i - 1) as usize];

            for j in 0..(previous_row.len() - 1) {
                r.push(previous_row[j] + previous_row[j+1]);
            }
            r.push(1);
            ans.push(r); // r의 소유권 이동
        }
        ans
    }
}