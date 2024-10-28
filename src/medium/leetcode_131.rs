use std::sync::mpsc::channel;

struct Solution;

// iter() : 참조를 빌려와서 반복함.
// into_iter() : 소유권을 가져와서 반복함. -> map에서는 필요할 듯.

impl Solution {

    fn dfs(s: &Vec<char>,
           start: usize,
           acc: &mut Vec<Vec<char>>,
           result: &mut Vec<Vec<String>>,
           dp: &mut Vec<Vec<bool>>
    ) {
        if start == s.len() {
            let temp = acc.iter()
                .map(|v| v.into_iter().collect())
                .collect();
            result.push(temp);
        }
        for end in start..s.len() {
            if s[start] == s[end] &&
                (end - start <= 2 || dp[start + 1][end - 1]) {
                dp[start][end] = true;

                let x: Vec<char> = s[start..end + 1]
                    .iter() // 참조자가 반환되는데
                    .copied() // 새로운 값이 필요하므로 여기서 복사.
                    .collect();

                acc.push(x);
                Self::dfs(s, end + 1, acc, result, dp);
                acc.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut result: Vec<Vec<String>> = vec![];

        let mut acc: Vec<Vec<char>> = vec![];

        let chars: Vec<char> = s.as_str().chars().collect();
        Self::dfs(&chars, 0, &mut acc, &mut result, &mut dp);

        result
    }
}