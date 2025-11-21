use std::iter::successors;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut all = successors(head.as_ref(), |n| n.next.as_ref())
            .map(|n| n.val)
            .collect::<Vec<_>>();
        all[left as usize - 1..right as usize].reverse();
        let mut dummy = ListNode::new(-1);
        all.into_iter().fold(&mut dummy, |cur, v| {
            cur.next = Some(Box::new(ListNode::new(v)));
            cur.next.as_mut().unwrap()
        });
        dummy.next
    }
}