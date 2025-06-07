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

        let mut tail = Box::new(ListNode::new(0));
        let mut tail_ref = tail.as_mut();
        let mut carry = 0;

        while p1.is_some() || p2.is_some()  || carry > 0 {

            let mut temp_sum = carry;
            if let Some(v1) = p1 {
                temp_sum += v1.val.clone();
                p1 = v1.next.as_ref();
            }

            if let Some(v2) = p2 {
                temp_sum += v2.val.clone();
                p2 = v2.next.as_ref();
            }

            carry = temp_sum / 10;
            temp_sum = temp_sum % 10;
            tail_ref.val = temp_sum;

            if p1.is_some() || p2.is_some() || carry > 0 {
                tail_ref.next = Some(Box::new(ListNode::new(0)));
                tail_ref = tail_ref.next.as_mut().unwrap();
            }

        }

        Some(tail)
    }
}

