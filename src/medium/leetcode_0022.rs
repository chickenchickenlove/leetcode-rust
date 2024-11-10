struct Solution;


use std::collections::HashSet;
impl Solution {

    fn get_wrap(v: Vec<String>) -> Vec<String> {
        let mut hs: HashSet<String> = HashSet::new();
        for s in &v {
            for i in 0..s.len() {
                let mut new_string = String::from("");
                new_string.push('(');
                new_string.push_str(&s[..i]);
                new_string.push(')');
                new_string.push_str(&s[i..]);
                hs.insert(new_string);
            }
        }

        hs.into_iter().collect()
    }


    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut v = vec![String::from("()")];

        if n == 1 {
            return v;
        }

        for i in 1..n {
            v = Self::get_wrap(v);
        }
        v
    }
}