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

        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut carry = 0;

        let mut first_node = Box::new(ListNode::new(0));
        let mut tail_ref = &mut first_node;

        while p1.is_some() || p2.is_some() || carry > 0 {
            let mut var = carry;

            if let Some(v) = p1 {
                var += v.val;
                p1 = v.next.as_ref();
            }

            if let Some(v) = p2 {
                var += v.val;
                p2 = v.next.as_ref();
            }

            carry = var / 10;
            var = var % 10;
            tail_ref.val = var;

            if p1.is_some() || p2.is_some() || carry > 0 {
                tail_ref.next = Some(Box::new(ListNode::new(0)));
                tail_ref = tail_ref.next.as_mut().unwrap();
            }
        }

        Some(first_node)

    }
}