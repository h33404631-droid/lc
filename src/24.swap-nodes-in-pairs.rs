use std::time::Duration;

use super::prelude::*;

// @lc app=leetcode id=24 lang=rust
//
// [24] Swap Nodes in Pairs
//
// https://leetcode.com/problems/swap-nodes-in-pairs/description/
//
// algorithms
// Medium (67.82%)
// Likes:    12729
// Dislikes: 492
// Total Accepted:    1.7M
// Total Submissions: 2.5M
// Testcase Example:  '[1,2,3,4]'
//
// Given a linked list, swap every two adjacent nodes and return its head. You
// must solve the problem without modifying the values in the list's nodes
// (i.e., only nodes themselves may be changed.)
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4]
//
// Output: [2,1,4,3]
//
// Explanation:
//
//
//
//
// Example 2:
//
//
// Input: head = []
//
// Output: []
//
//
// Example 3:
//
//
// Input: head = [1]
//
// Output: [1]
//
//
// Example 4:
//
//
// Input: head = [1,2,3]
//
// Output: [2,1,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 100].
// 0 <= Node.val <= 100
//
//

// @lc code=start
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val:  0,
            next: head,
        });
        let mut prev = &mut dummy;
        while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
            // Get the two nodes to swap
            let mut first = prev.next.take().unwrap();
            let mut second = first.next.take().unwrap();
            first.next = second.next.take();
            second.next = Some(first);
            prev.next = Some(second);

            prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy.next
    }
}
// @lc code=end
