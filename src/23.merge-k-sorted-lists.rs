use super::prelude::*;

// @lc app=leetcode id=23 lang=rust
//
// [23] Merge k Sorted Lists
//
// https://leetcode.com/problems/merge-k-sorted-lists/description/
//
// algorithms
// Hard (57.56%)
// Likes:    20724
// Dislikes: 768
// Total Accepted:    2.7M
// Total Submissions: 4.6M
// Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
//
// You are given an array of k linked-lists lists, each linked-list is sorted
// in ascending order.
//
// Merge all the linked-lists into one sorted linked-list and return it.
//
//
// Example 1:
//
//
// Input: lists = [[1,4,5],[1,3,4],[2,6]]
// Output: [1,1,2,3,4,4,5,6]
// Explanation: The linked-lists are:
// [
// ⁠ 1->4->5,
// ⁠ 1->3->4,
// ⁠ 2->6
// ]
// merging them into one sorted linked list:
// 1->1->2->3->4->4->5->6
//
//
// Example 2:
//
//
// Input: lists = []
// Output: []
//
//
// Example 3:
//
//
// Input: lists = [[]]
// Output: []
//
//
//
// Constraints:
//
//
// k == lists.length
// 0 <= k <= 10^4
// 0 <= lists[i].length <= 500
// -10^4 <= lists[i][j] <= 10^4
// lists[i] is sorted in ascending order.
// The sum of lists[i].length will not exceed 10^4.
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::{cmp::Reverse, collections::BinaryHeap};

        let mut heap = BinaryHeap::new();

        impl PartialOrd for ListNode {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for ListNode {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.val.cmp(&other.val) }
        }

        for i in lists {
            if let Some(n) = i {
                heap.push(Reverse(n));
            }
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        while let Some(Reverse(node)) = heap.pop() {
            current.next = Some(node);
            current = current.next.as_mut().unwrap();
            // 将当前节点的下一个节点（如果存在）推入堆中
            if let Some(next_node) = current.next.take() {
                heap.push(Reverse(next_node));
            }
        }

        dummy.next
    }
}
// @lc code=end
