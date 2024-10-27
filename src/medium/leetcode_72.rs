impl Solution {

    fn sol(word1: &Vec<char>, word2: &Vec<char>, i1: i32, i2: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i1 == -1 {
            return i2 - i1;
        }
        if i2 == -1 {
            return i1 - i2;
        }

        let (i1, i2) = (i1 as usize, i2 as usize);

        if dp[i1][i2] != i32::MAX {
            return dp[i1][i2];
        }

        if word1[i1] == word2[i2] {
            dp[i1][i2] = Self::sol(word1, word2, i1 as i32- 1, i2 as i32 - 1, dp);
        }
        else {
            let insert = Self::sol(word1, word2, i1 as i32, i2 as i32 - 1, dp);
            let delete = Self::sol(word1, word2, i1 as i32 - 1, i2 as i32, dp);
            let replace = Self::sol(word1, word2, i1 as i32 - 1, i2 as i32 - 1, dp);
            dp[i1][i2] = insert.min(delete).min(replace) + 1;
        }

        dp[i1][i2]
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {

        let word1_vec: Vec<char> = word1.chars().collect();
        let word2_vec: Vec<char> = word2.chars().collect();

        let mut dp = vec![vec![i32::MAX; word2.len()]; word1.len()];

        Self::sol(&word1_vec,
                  &word2_vec,
                  word1.len() as i32 - 1,
                  word2.len() as i32 - 1,
                  &mut dp)
    }
}


pub struct Solution;


