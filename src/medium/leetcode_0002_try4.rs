
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

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut carry = 0;

        let mut tail: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail_ref = &mut tail;

        while p1.is_some() || p2.is_some() || carry > 0 {
            let mut temp_sum = carry;
            if let Some(v) = p1 {
                temp_sum += v.val;
                p1 = v.next.as_ref();
            }

            if let Some(v) = p2 {
                temp_sum += v.val;
                p2 = v.next.as_ref();
            }

            carry = temp_sum / 10;
            temp_sum = temp_sum % 10;

            tail_ref.val = temp_sum;

            if p1.is_some() || p2.is_some() || carry > 0 {
                let new: Box<ListNode> = Box::new(ListNode::new(0));
                tail_ref.next = Some(new);
                tail_ref = tail_ref.next.as_mut().unwrap();
            }
        }

        Some(tail)

    }
}

