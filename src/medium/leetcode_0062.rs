
pub struct Solution;

use std::collections::VecDeque;

impl Solution {

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let moves: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 0 )
        ];

        let (m, n) = (m as usize, n as usize);

        // 이렇게 개선할 수 있음..
        let mut d = vec![vec![0; n]; m];
        let mut v = vec![vec![false; n]; m];

        d[0][0] = 1;

        let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
        deque.push_back((0, 0));

        while let Some((r, c)) = deque.pop_front() {
            if v[r][c] {
                continue;
            }
            v[r][c] = true;

            for (mr, mc) in &moves {
                let next_r = mr + r;
                let next_c = mc + c;

                if next_r < m && next_c < n {
                    d[next_r][next_c] += d[r][c];
                    deque.push_back((next_r, next_c));
                }
            }
        }
        d[m-1][n-1]
    }
}