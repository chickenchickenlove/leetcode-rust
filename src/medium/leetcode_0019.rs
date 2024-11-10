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
        let mut visit: Vec<Box<ListNode>> = vec![];
        let mut start = &head;

        while let Some(h) = start {
            visit.push(h.clone());
            start = &h.next;
        }

        let t = visit.len() - n as usize;
        if t == 0 && visit.len() == 1 {
            None
        }
        else if t == 0 && visit.len() > 1 {
            Some(visit[t+1].clone())
        }
        else {
            // visit.remove(t);

            let mut new_head = Box::new(ListNode::new(visit[0].val));
            let mut start_ref = &mut new_head;
            for i in 1..visit.len() {
                if i == t {
                    continue;
                }
                let mut now_node = ListNode::new(visit[i].val);
                start_ref.next = Some(Box::new(now_node)); // now_node의 소유권이 Box로 이동됨.
                let next_ref = start_ref.next.as_mut().unwrap();
                start_ref = next_ref;
            }

            Some(new_head)
        }
    }
}

fn main() {
    println!("{:?}", Solution::remove_nth_from_end(None, 1));
}