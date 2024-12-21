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

        let mut node1 = l1;
        let mut node2 = l2;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut mut_ref = &mut result;
        let mut carry = 0;

        // is_some 이후에 바로 SharedReference가 제거됨.
        while node1.is_some() || node2.is_some() || carry > 0 {
            let mut t_sum = carry;
            carry = 0;

            if let Some(node) = node1 {
                t_sum += node.val;
                // 따라서 이 곳의 코드가 실행되는데 문제가 없어짐.
                node1 = node.next;
            }

            if let Some(node) = node2 {
                t_sum += node.val;
                node2 = node.next;
            }

            if t_sum >= 10 {
                carry = 1;
                t_sum -= 10;
            }

            if let Some(node) = mut_ref {
                node.val = t_sum;
            }

            if node1.is_some() || node2.is_some() || carry > 0 {
                if let Some(node) = mut_ref {
                    node.next = Some(Box::new(ListNode::new(0)));
                    mut_ref = &mut node.next;
                }
            }
        }

        result
    }
}