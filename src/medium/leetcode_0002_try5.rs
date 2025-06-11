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
        let mut tail_own: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail = &mut tail_own;

        let mut carry = 0;
        // &l1이나, l1.as_ref() 상관없다.
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();

        while p1.is_some() || p2.is_some() || carry > 0 {
            let mut temp_sump = carry;
            if let Some(value) = p1{
                temp_sump += value.val;
                p1 = value.next.as_ref();
            }

            if let Some(value) = p2 {
                temp_sump += value.val;
                p2 = value.next.as_ref();
            }

            carry = temp_sump / 10;
            temp_sump = temp_sump % 10;

            tail.val = temp_sump;

            if p1.is_some() || p2.is_some() || carry > 0 {
                tail.next = Some(Box::new(ListNode::new(0)));
                tail = tail.next.as_mut().unwrap();
            }
        }

        Some(tail_own)

    }
}