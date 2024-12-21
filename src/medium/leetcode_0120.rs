use std::cmp::min;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        if n == 1 {
            return triangle[0][0];
        }

        let mut new_triangle = triangle.clone();

        for i in 1..n {
            for j in 0..new_triangle[i].len() {
                let now_value = new_triangle[i][j];

                new_triangle[i][j] =
                    if j == 0 {
                        new_triangle[i - 1][0] + now_value
                    } else if j == triangle[i].len() - 1 {
                        let last_idx = new_triangle[i - 1].len() - 1;
                        new_triangle[i - 1][last_idx] + now_value
                    } else {
                        min(new_triangle[i - 1][j - 1], new_triangle[i - 1][j]) + now_value
                    };
            }
        }

        let last_idx = new_triangle.len() - 1;

        new_triangle[last_idx]
            .iter()
            .cloned()
            .min()
            .unwrap_or_default()
    }
}