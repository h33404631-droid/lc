use super::prelude::*;

/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (46.86%)
 * Likes:    34678
 * Dislikes: 6937
 * Total Accepted:    6.2M
 * Total Submissions: 13.1M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 *
 * Example 1:
 *
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 *
 *
 */

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = Some(Box::new(ListNode::new(-1))); // this a dummy node
        let mut cur = &mut dummy;
        let mut carry = 0;
        loop {
            if l1.is_none() && l2.is_none() {
                break;
            }
            let mut n1_val = 0;
            while let Some(n1) = l1 {
                l1 = n1.next;
                n1_val = n1.val;
                break;
            }
            let mut n2_val = 0;
            while let Some(n2) = l2 {
                l2 = n2.next;
                n2_val = n2.val;
                break;
            }
            let mut new_val = carry + n2_val + n1_val;
            if new_val >= 10 {
                new_val = new_val - 10;
                carry = 1;
            } else {
                carry = 0;
            }
            let new_node = Some(Box::new(ListNode::new(new_val)));
            cur.as_mut().unwrap().next = new_node;
            cur = &mut cur.as_mut().unwrap().next;
        }
        if carry == 1 {
            let new_node = Some(Box::new(ListNode::new(1)));
            cur.as_mut().unwrap().next = new_node;
            // cur = &mut cur.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}
// @lc code=end
