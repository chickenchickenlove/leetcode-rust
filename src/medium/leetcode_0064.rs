pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let lr = grid.len();
        let lc = (&grid[0]).len();

        let mut c_grid = grid.clone();
        let g = &mut c_grid;

        for r in 0..lr {
            for c in 0..lc{
                if (r == 0 && c == 0) {
                    continue;
                }

                let ir = r as i32;
                let ic = c as i32;

                match (-1 < r as i32 -1, -1 < c as i32-1) {
                    (true, true) => g[r][c] = std::cmp::min(
                        g[r-1][c] + g[r][c],
                        g[r][c-1] + g[r][c]),
                    (false, _) => g[r][c] = g[r][c-1] + g[r][c],
                    (_, false) => g[r][c] = g[r-1][c] + g[r][c],
                }
            }
        }

        g[lr - 1][lc - 1]
    }
}