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
        let mut mutable_head = head;
        let mut ref_head = &mutable_head;
        let mut vec = vec![];

        while let Some(node) = ref_head {
            ref_head = &node.next;
            vec.push(node.val);
        }

        if vec.len() == 1 {
            return None;
        }

        let target_index = (vec.len() - n as usize);
        vec.remove(target_index);

        let mut result = Some(Box::new(ListNode::new(0)));
        let mut result_ref: &mut Option<Box<ListNode>> = &mut result;

        for (idx, value) in vec.into_iter().enumerate() {
            if idx == 0 {
                result_ref.as_mut().unwrap().val = value;
            } else {
                let mut new_node = Some(Box::new(ListNode::new(value)));
                let now_node = result_ref.as_mut().unwrap();

                now_node.next = new_node;
                result_ref = &mut now_node.next;
            }
        }

        result
    }
}