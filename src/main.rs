use std::io::stdout;
// use crate::easy::leetcode_1::Solution;
// use crate::easy::leetcode_2::{ListNode, Solution};
// use crate::easy::leetcode_9::Solution;

mod easy;
mod medium;


use crate::medium::leetcode_0019_try2::{ListNode, Solution};


fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    a.next = Some(b);

    Solution::remove_nth_from_end(Some(a), 1);

    // medium::leetcode_15::Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    // let r = medium::leetcode_16::Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);
    //
    // let a = "hello";
    // let b = a.chars();
    // // let c = a[1];

    // let r = medium::leetcode_72::Solution::min_distance(String::from(""), String::from(""));
    // println!("{}", r);


    // let r = medium::leetcode_62::Solution::unique_paths(3, 2);
    // println!("{}", r);

    // let k = String::from("Hello");
    // let chars: Vec<char> = k.as_str().chars().collect();
    // let k1 = &chars;
    // let k2 = &k1[0..k1.len()-1];
    // let k3: &Vec<char> = &k1[0..k1.len() - 1].iter().copied().collect();
    // let k4 = ['h', 'e', 'l', 'l', 'o'];
    // let k5: Vec<char> = k4.iter().collect();
    //
    // println!("123", );


    // let a = easy::leetcode_125::Solution::is_palindrome(String::from(" "));
    // println!("{}", a);

}



// for leetcode_9
// fn main() {
//     // ListNode 인스턴스를 생성할 때, Box::new(...)를 사용하여 힙 메모리에 데이터를 저장하고, 이를 next field에서 참조할 수 있도록 바꿈.
//     // mut으로 선언하면 next 필드를 수정하게 했음. 러스트에서는 변수와 구조체 필드가 기본적으로 불변임.
//     let mut node1 = Box::new(ListNode::new(9));
//     let mut node2 = Box::new(ListNode::new(9));
//     let mut node3 = Box::new(ListNode::new(9));
//     let mut node4 = Box::new(ListNode::new(9));
//     let mut node5 = Box::new(ListNode::new(9));
//     let mut node6 = Box::new(ListNode::new(9));
//     let mut node7 = Box::new(ListNode::new(9));
//
//     // Some은 Option 타입의 일부로 값이 존재하는 경우를 나타내는 열거형 변형임.
//     // Some, None 가능함.
//     node6.next = Some(node7);
//     node5.next = Some(node6);
//     node4.next = Some(node5);
//     node3.next = Some(node4);
//     node2.next = Some(node3);
//     node1.next = Some(node2);
//
//
//     let mut node_1 = Box::new(ListNode::new(9));
//     let mut node_2 = Box::new(ListNode::new(9));
//     let mut node_3 = Box::new(ListNode::new(9));
//     let mut node_4 = Box::new(ListNode::new(9));
//
//     node_3.next = Some(node_4);
//     node_2.next = Some(node_3);
//     node_1.next = Some(node_2);
//
//
//     let option = Solution::add_two_numbers(Some(node1), Some(node_1));
//     let a = 1;
//     println!("hello");
//     // println!("{}", option.as_ref().unwrap().val);
//
//
// }


// fn main() {
//     let g1 = String::from("HI");
//     show_message1(&g1);
//     println!("{}", g1);
// }
//
// fn show_message1(msg: &String) {
//     println!("{}", msg);
// }


// fn main() {
//     let msg = gen_message();
//     println!("{}", msg);
// }
//
//
// fn gen_message() -> &str {
//     let msg = String::from("HI");
//     return &msg;
// }


// fn main() {
//     let mut g1 = String::from("HI");
//     show_message1(&mut g1);
//     println!("{}", g1);
// }
//
// fn show_message1(msg: &mut String) {
//     println!("{}", msg);
//     msg.insert(0, '"');
//     msg.push('"');
// }
//
//
