// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut cl1 = &l1;
        let mut cl2 = &l2;
        let mut carry = 0;
        let mut result = Box::new(ListNode::new(0));
        let mut current_node = &mut result;

        while cl1.is_some() || cl2.is_some() || carry > 0 {
            current_node.val += carry;
            carry = 0;
            if let Some(v) = cl1 {
                current_node.val += v.val;
                cl1 = &v.next;
            }
            if let Some(v) = cl2 {
                current_node.val += v.val;
                cl2 = &v.next;
            }

            if current_node.val >= 10 {
                current_node.val -= 10;
                carry = 1;
            }

            if cl1.is_some() || cl2.is_some() || carry > 0 {
                let mut next_node = Box::new(ListNode::new(0));
                current_node.next = Some(next_node);
                current_node = current_node.next.as_mut().unwrap();
            }

        };

        Some(result)
    }
}