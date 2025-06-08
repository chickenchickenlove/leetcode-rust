use std::collections::VecDeque;

struct Solution;

impl Solution {


    fn dfs(jump_map: Vec<i32>, start: i32, end: &Vec<usize>) -> bool {
        let mut visit_list = vec![0; jump_map.len()];
        let mut end_list = vec![0; jump_map.len()];

        for &e in end.iter() {
            end_list[e] = 1;
        }

        let mut que = VecDeque::from([start as usize]);

        while !que.is_empty() {
            let now_position = que.pop_front().unwrap();
            let jump_range = jump_map[now_position] as usize;

            if end_list[now_position] == 1 {
                return true;
            }

            let next_position_nega  = now_position as i32 - jump_range as i32 ;
            if next_position_nega >= 0 && visit_list[next_position_nega as usize] == 0 {
                visit_list[next_position_nega as usize] = 1;
                que.push_back(next_position_nega as usize);
            }

            let next_position_posi = now_position + jump_range;
            if next_position_posi < visit_list.len() && visit_list[next_position_posi] == 0 {
                visit_list[next_position_posi] = 1;
                que.push_back(next_position_posi);
            }
        }
        false
    }


    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut end: Vec<usize> = vec![];

        for (i, &n) in arr.iter().enumerate() {
            if n == 0 {
                end.push(i);
            }
        }

        Self::dfs(arr, start, &end)
    }
}