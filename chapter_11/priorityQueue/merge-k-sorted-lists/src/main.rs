use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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


pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut priority_queue = std::collections::BinaryHeap::new();
    for list in lists {
        if let Some(l) = list {
            priority_queue.push(l);
        }
    }
    let mut dummy = Box::new(ListNode::new(0));
    let mut cur = &mut dummy;

    while let Some(mut node) = priority_queue.pop() {
        if let Some(next) = node.next.take() {
            priority_queue.push(next);
        }
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}