
struct Solution;
impl Solution {

    pub fn get_func(number: usize, length: usize) -> usize{
        if number >= length {
            number - length
        }
        else {
            number
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums;
        let (mut l, mut r) = (0, n.len() - 1);
        let mut max_value = n[l].max(n[r]);
        let mut max_value_idx = if n[l] > n[r] { l } else {r};

        while l <= r {
            let mut m = (l + r) / 2;
            if n[l] > n[m].max(n[r]) {
                if n[l] > max_value {
                    max_value = n[l];
                    max_value_idx = l;
                }
                r = m - 1;
            }
            else {
                if n[m] > n[r] && n[m] > max_value {
                    max_value = n[m];
                    max_value_idx = m;
                }
                else if n[m] < n[r] && n[r] > max_value {
                    max_value = n[r];
                    max_value_idx = r;
                }
                l = m + 1;
            }
        }

        l = max_value_idx + 1;
        r = max_value_idx + n.len();

        while l <= r {
            let mut m = (l + r) / 2;
            let mut ml = Self::get_func(m, n.len());
            if n[ml] == target {
                return ml as i32;
            }
            else if n[ml] < target {
                l = m + 1;
            }
            else {
                r = m - 1;
            }
        }
        return -1
    }
}