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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = l1;
        let mut l_ref1 = &list1;
        let mut list2 = l2;
        let mut l_ref2 = &list2;


        let mut carry = 0;
        let mut sum = 0;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut result_ref: &mut Option<Box<ListNode>> = &mut result;

        while l_ref1.is_some() || l_ref2.is_some() || carry > 0 {

            if let Some(n1) = l_ref1 {
                sum += n1.val;
                l_ref1 = &n1.next;
            }

            if let Some(n2) = l_ref2 {
                sum += n2.val;
                l_ref2 = &n2.next;
            }

            sum += carry;
            carry = 0;

            if sum >= 10 {
                sum -= 10;
                carry += 1;
            }

            let k = result_ref;
            let k1 = result_ref.as_mut();
            let now_node = result_ref.as_mut().unwrap();
            now_node.val = sum;
            sum = 0;

            if l_ref1.is_some() || l_ref2.is_some() || carry > 0 {
                let mut next_node = Some(Box::new(ListNode::new(0)));
                now_node.next = next_node;
                result_ref = &mut now_node.next;
            }

        }

        result
    }
}