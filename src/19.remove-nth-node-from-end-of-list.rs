use std::{cell::RefCell, rc::Rc};

use super::prelude::*;

// @lc app=leetcode id=19 lang=rust
//
// [19] Remove Nth Node From End of List
//
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
//
// algorithms
// Medium (49.75%)
// Likes:    20463
// Dislikes: 872
// Total Accepted:    3.7M
// Total Submissions: 7.5M
// Testcase Example:  '[1,2,3,4,5]\n2'
//
// Given the head of a linked list, remove the n^th node from the end of the
// list and return its head.
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4,5], n = 2
// Output: [1,2,3,5]
//
//
// Example 2:
//
//
// Input: head = [1], n = 1
// Output: []
//
//
// Example 3:
//
//
// Input: head = [1,2], n = 1
// Output: [1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is sz.
// 1 <= sz <= 30
// 0 <= Node.val <= 100
// 1 <= n <= sz
//
//
//
// Follow up: Could you do this in one pass?
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val:  0,
            next: head,
        });

        // 计算链表长度
        let mut length = 0;
        let mut current = dummy.next.as_ref();
        while let Some(node) = current {
            length += 1;
            current = node.next.as_ref();
        }

        // 找到要删除节点的前一个节点的位置
        let target_pos = length - n;
        let mut current = &mut dummy;

        for _ in 0..target_pos {
            current = current.next.as_mut().unwrap();
        }

        // 删除节点
        let next_node = current.next.as_mut().unwrap().next.take();
        current.next = next_node;

        dummy.next
    }
}
// @lc code=end
