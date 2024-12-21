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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut nums = vec![];
        let mut mut_head = head;

        while let Some(node) = mut_head {
            nums.push(node.val);
            mut_head = node.next;
        }

        if nums.len() == 1 as usize {
            return None;
        }

        let should_remove_idx = (n - 1) as usize;
        let mut result = Some(Box::new(ListNode::new(0)));

        for (index, value) in nums.iter().rev().enumerate() {
            if index == should_remove_idx {
                continue;
            }

            if let Some(node) = &mut result {
                node.val = value.clone();
            }

            if (index != nums.len() - 1) {
                if (index + 1 == nums.len() - 1) && (index + 1 == should_remove_idx) {
                    continue;
                }

                let mut next_node = Some(Box::new(ListNode::new(0)));
                if let Some(node) = &mut next_node {
                    node.next = result;
                }
                result = next_node;
            }

        };
        result
    }
}