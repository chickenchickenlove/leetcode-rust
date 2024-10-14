pub struct Solution;

// try2
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string = x.to_string();
        let length = string.len();

        let chars: Vec<char> = string.chars().collect();

        if length == 1 {
            return true;
        }

        let end = length/2 - 1;

        for i in 0..=end {
            if chars[i] != chars[length - 1 - i] {
                return false;
            }
        }
        return true;
    }
}



// try2
// impl Solution {
//     pub fn is_palindrome(x: i32) -> bool {
//         let num_str = x.to_string();
//         let chars: Vec<char> = num_str.chars().collect();
//
//         if num_str.len() == 1 {
//             return true
//         };
//
//         let end = (num_str.len() / 2) - 1;
//         for i in 0..=end {
//             if (chars[i] != chars[num_str.len()-1-i]) {
//                 return false;
//             }
//         }
//         return true
//     }
// }