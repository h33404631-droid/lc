use super::prelude::*;

// @lc app=leetcode id=206 lang=rust
//
// [206] Reverse Linked List
//
// https://leetcode.com/problems/reverse-linked-list/description/
//
// algorithms
// Easy (79.65%)
// Likes:    23553
// Dislikes: 550
// Total Accepted:    5.6M
// Total Submissions: 7.1M
// Testcase Example:  '[1,2,3,4,5]'
//
// Given the head of a singly linked list, reverse the list, and return the
// reversed list.
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
//
//
// Example 2:
//
//
// Input: head = [1,2]
// Output: [2,1]
//
//
// Example 3:
//
//
// Input: head = []
// Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000
//
//
//
// Follow up: A linked list can be reversed either iteratively or recursively.
// Could you implement both?
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}
}
// @lc code=end
