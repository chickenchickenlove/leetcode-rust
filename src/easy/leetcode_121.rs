pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut mv = 999999999;

        for &price in prices.iter() {
            mv = std::cmp::min(price, mv);
            result = std::cmp::max(result, price - mv);
        }
        result
    }
}