use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle.clone();
        let n = triangle.len();

        if n == 1 {
            return triangle[0][0];
        };

        for i in 1..n {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] += triangle[i - 1][0];
                } else if j == triangle[i].len() - 1 {
                    triangle[i][j] += triangle[i - 1][triangle[i-1].len()-1];
                } else {
                    triangle[i][j] += min(triangle[i - 1][j - 1], triangle[i - 1][j]);
                }
            }
        }

        *triangle
            .last()
            .expect("error")
            .iter()
            .min()
            .expect("error")
    }
}