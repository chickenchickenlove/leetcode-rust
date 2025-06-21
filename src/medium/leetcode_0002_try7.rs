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

      let mut result: Box<ListNode> = Box::new(ListNode::new(0));
      let mut tail = &mut result;
      let mut carry = 0;

      while p1.is_some() || p2.is_some() || carry > 0 {
        let mut temp = carry;
        if let Some(v) = p1 {
          temp += v.val;
          p1 = v.next.as_ref();
        }

        if let Some(v) = p2 {
          temp += v.val;
          p2 = v.next.as_ref();
        }

        carry = temp / 10;
        tail.val = temp % 10;

        if p1.is_some() || p2.is_some() || carry > 0 {
          let mut new_node = Box::new(ListNode::new(0));
          tail.next = Some(new_node);
          tail = tail.next.as_mut().unwrap();
        }

      }

      Some(result)

    }
}