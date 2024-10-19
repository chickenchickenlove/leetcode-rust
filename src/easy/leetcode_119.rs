
struct Solution;

// Input: rowIndex = 3
// Output: [1,3,3,1]


impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ans = vec![1];

        for i in 1..=row_index {
            let mut r = vec![1];
            for j in 0..(ans.len()-1) {
                r.push(ans[j] + ans[j + 1]);
            }
            r.push(1);
            ans = r;
        }
        ans
    }
}