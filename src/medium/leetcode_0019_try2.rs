// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut visit: Vec<i32> = vec![];

        let mut length: usize = 0;
        let mut start1 = &head;
        while let Some(v) = start1 {
            length += 1;
            start1 = &v.next;
        }

        let target_idx = length - n as usize;

        let mut cnt: usize = 0;
        let mut start2 = &head;
        while let Some(v) = start2 {
            if cnt != target_idx {
                visit.push(v.val);
            }
            cnt += 1;
            start2 = &v.next;
        }

        if visit.len() == 0 {
            None
        } else{
            let mut result = Box::new(ListNode::new(visit[0]));
            let mut head = &mut result;
            for i in 1..visit.len() {
                head.next = Some(Box::new(ListNode::new(visit[i])));
                head = head.next.as_mut().unwrap();
            }
            Some(result)
        }
    }
}