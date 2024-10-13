pub struct Solution;
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub(crate) fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node1 = l1;
        let mut current_node2 = l2;

        let mut result_node = Box::new(ListNode::new(0));
        let mut current_node = &mut result_node;

        let mut carry = 0;

        while current_node1.is_some() || current_node2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = current_node1 {
                sum += node.val;
                current_node1 = node.next;
            }

            if let Some(node) = current_node2 {
                sum += node.val;
                current_node2 = node.next;
            }

            current_node.val = sum % 10;
            carry = sum / 10;

            if (current_node1.is_some() || current_node2.is_some() || carry > 0) {
                current_node.next = Some(Box::new(ListNode::new(0)));
                current_node = current_node.next.as_mut().unwrap();
            }
        }

        return Some(result_node);
    }
}



// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut current_node1 = l1;
//         let mut current_node2 = l2;
//
//         let mut result_node = Box::new(ListNode::new(0));
//         let mut current_node = &mut result_node;
//
//         let mut carry = 0;
//
//         while true {
//             println!("hello10");
//             if (current_node1.is_some()) {
//                 current_node.val += current_node1.as_ref().unwrap().val;
//
//                 if (current_node1.as_ref().unwrap().next.is_some()){
//                     current_node1 = current_node1.take().unwrap().next;
//                 } else {
//                     current_node1 = None;
//                 }
//             }
//
//             if (current_node2.is_some()) {
//                 current_node.val += current_node2.as_ref().unwrap().val;
//
//                 if (current_node2.as_ref().unwrap().next.is_some()){
//                     current_node2 = current_node2.take().unwrap().next;
//                 } else {
//                     current_node2 = None;
//                 }
//
//             }
//
//             current_node.val += carry;
//             carry = 0;
//
//             if (current_node.val >= 10) {
//                 current_node.val -= 10;
//                 carry = 1;
//             }
//
//             if (current_node1.is_none() && current_node2.is_none()){
//                 break;
//             }
//
//             let next_node = Box::new(ListNode::new(0));
//             current_node.next = Some(next_node);
//             current_node = current_node.next.as_mut().unwrap();
//         }
//
//         println!("here");
//
//
//         if (carry > 0) {
//             let next_node = Box::new(ListNode::new(carry));
//             current_node.next = Some(next_node);
//             current_node = current_node.next.as_mut().unwrap();
//             carry = 0;
//         }
//
//         return Some(result_node);
//     }
// }