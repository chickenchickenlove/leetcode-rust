pub fn sol(i: usize, n: usize, p: &Vec<i32>) -> i32{
    if i == 0 {
        return 0;
    }

    let pre_v = sol(i - 1, n, p); // P가 불변 참조자가 아니면, 여기서 소유권이 넘어감. 따라서 뒷쪽에 있는 라인에서는 p[i], p[0]가 무효화 된다.
    let diff = p[i] - p[i - 1];
    let start_diff = p[i] - p[0];

    vec![pre_v, pre_v + diff, start_diff]
        .iter()
        .cloned()
        .max()
        .unwrap_or_default()
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 1 {
            return 0;
        }
        sol(n-1, n, &prices)
    }
}