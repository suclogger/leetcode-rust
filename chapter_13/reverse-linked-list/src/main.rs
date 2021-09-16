use std::ops::{Deref, DerefMut};

fn main() {
    println!("Hello, world!");
}
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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut pre = Option::None;
    while let Some(mut cur) = head {
        head = (*cur).next;
        (*cur).next = pre;
        pre = Option::Some(cur);
    }
    pre
}