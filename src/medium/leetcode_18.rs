

pub struct Solution;


use std::collections::HashSet;
impl Solution {

    fn judge(a: usize, b: usize, l: usize, r: usize) -> bool {
        (a != b) && (a != l) && (a != r) && (b != l) && (b != r) && (l != r)
    }

    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cnums = nums.clone(); // nums의 소유권은 이동 가능하나, immutable이기 때문에 sort를 위해 clone함.
        cnums.sort();

        let mut targets = vec![];
        for &n in &cnums {
            targets.push(target - n); // n의 소유권은 이동되지 않음. 왜냐면 역참조이기 때문임.
        }

        let length = nums.len();
        let mut result = HashSet::new();

        for (a, &t) in targets.iter().enumerate() {
            if a>0 && cnums[a] == cnums[a-1] {
                continue;
            }

            for b in a+1.. length {
                let (mut l, mut r) = (b + 1, length - 1);
                while l < r {
                    let ts: i64 = cnums[l] as i64 + cnums[r] as i64 + cnums[b] as i64; // i32를 다 더하면 i64가 될 수도 있기 때문에 이렇게 수정.
                    if ts == t as i64 && Self::judge(a, b, l, r) { // ** Copy Trait을 구현하지 않았다면 a,b,l,r의 소유권이 이동함.
                        let mut tl = [cnums[a], cnums[b], cnums[l], cnums[r]]; // ** Copy Trait을 구현하지 않았다면, cnums[a]의 소유권이 이동함.
                        tl.sort();
                        result.insert(tl);
                    }
                    if ts < t as i64{
                        l += 1;
                    }
                    else {
                        r -= 1;
                    }
                }
            }
        }
        result.into_iter()₩
            .map(|t| t.into_iter().collect()) // 여기서 t.iter()를 하게 되면 참조자이기 때문에 그 값은 필요없음. 그래서 into_iter()를 함.
            .collect()
    }
}