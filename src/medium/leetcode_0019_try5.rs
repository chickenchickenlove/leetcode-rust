
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
        let mut now_node_ref = &head;
        let mut v = vec![];

        while let Some(node) = now_node_ref {
            v.push(node.val);
            now_node_ref = &node.next;
        }

        if v.len() == 1 {
            return None
        }

        let should_ignore_index = v.len() - n as usize;
        let mut result = None;
        let mut result_ref: &mut Option<Box<ListNode>> = &mut result;

        for (index, value) in v.iter().enumerate() {
            if should_ignore_index != index {
                let mut now_node = Some(Box::new(ListNode::new(value.clone())));
                if let Some(previous_node) = result_ref {
                    previous_node.next = now_node;
                    // result_ref = &mut now_node; // 이렇게 작성하면 동작하지 않음. 위 코드라인에서 now_node는 이미 소모되었음.
                    result_ref = &mut previous_node.next;
                } else {
                    result = now_node;
                    result_ref = &mut result;
                }
            }
        }

        result
    }
}