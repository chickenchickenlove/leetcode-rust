use std::cmp::min;

struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX-1000; amount as usize + 1];
        dp[0] = 0;

        let mut new_coins: Vec<i32> = coins
            .into_iter()
            .clone()
            .collect();

        new_coins.sort();

        for &coin in &new_coins {
            if coin >= amount + 1 {
                continue
            }
            dp[coin as usize] = 1;
        }

        let amount_usize = amount as usize;
        for i  in new_coins[0] as usize..amount_usize + 1 {
            for &coin in &new_coins {
                let coin_usize = coin as usize;
                if i + coin_usize >= amount_usize + 1 {
                    break;
                }
                dp[i + coin_usize] = dp[i + coin_usize].min(dp[i] + 1);
            }
        }

        let mut result = dp[dp.len() - 1];
        if result == i32::MAX-1000 {
            -1
        } else {
            result
        }
    }
}