use super::prelude::*;

// @lc app=leetcode id=61 lang=rust
//
// [61] Rotate List
//
// https://leetcode.com/problems/rotate-list/description/
//
// algorithms
// Medium (40.41%)
// Likes:    10784
// Dislikes: 1521
// Total Accepted:    1.4M
// Total Submissions: 3.5M
// Testcase Example:  '[1,2,3,4,5]\n2'
//
// Given the head of a linked list, rotate the list to the right by k
// places.
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4,5], k = 2
// Output: [4,5,1,2,3]
//
//
// Example 2:
//
//
// Input: head = [0,1,2], k = 4
// Output: [2,0,1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 500].
// -100 <= Node.val <= 100
// 0 <= k <= 2 * 10^9
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 0 || head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;

        // 1. get the length of the linkedlist
        let mut len = 1;
        {
            let mut p = head.as_ref().unwrap();
            while let Some(ref node) = p.next {
                len += 1;
                p = node;
            }
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let step = len - k - 1;

        // 2. 找到 new_tail 的位置
        let mut idx = 0;
        let mut p = head.as_ref().unwrap();
        while idx < step {
            p = p.next.as_ref().unwrap();
            idx += 1;
        }

        // 3. 用可变借用找到 new_tail，并断开
        let mut new_tail = head.as_mut().unwrap();
        for _ in 0..step {
            new_tail = new_tail.next.as_mut().unwrap();
        }
        let mut new_head = new_tail.next.take();

        // 4. 找到 old_tail，并形成环
        let mut old_tail = new_head.as_mut().unwrap();
        while old_tail.next.is_some() {
            old_tail = old_tail.next.as_mut().unwrap();
        }
        old_tail.next = head;

        new_head
    }
}
// @lc code=end
