use super::prelude::*;

// @lc app=leetcode id=21 lang=rust
//
// [21] Merge Two Sorted Lists
//
// https://leetcode.com/problems/merge-two-sorted-lists/description/
//
// algorithms
// Easy (67.25%)
// Likes:    23970
// Dislikes: 2345
// Total Accepted:    5.6M
// Total Submissions: 8.3M
// Testcase Example:  '[1,2,4]\n[1,3,4]'
//
// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists into one sorted list. The list should be made by
// splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
// Example 1:
//
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
//
// Input: list1 = [], list2 = []
// Output: []
//
//
// Example 3:
//
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut l1 = list1;
        let mut l2 = list2;
        while l1.is_some() && l2.is_some() {
            // We know both are Some, so we can unwrap safely within the `if`.
            let n1 = l1.as_ref().unwrap();
            let n2 = l2.as_ref().unwrap();

            if n1.val <= n2.val {
                current.next = l1.take();
                current = current.next.as_mut().unwrap();
                l1 = current.next.take();
            } else {
                current.next = l2.take();
                current = current.next.as_mut().unwrap();
                l2 = current.next.take();
            }
        }
        dummy.next
    }
}
// @lc code=end
