use core::panic;

use super::prelude::*;

// @lc app=leetcode id=1317 lang=rust
//
// [1317] Convert Integer to the Sum of Two No-Zero Integers
//
// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/description/
//
// algorithms
// Easy (54.34%)
// Likes:    681
// Dislikes: 343
// Total Accepted:    126.4K
// Total Submissions: 220.5K
// Testcase Example:  '2'
//
// No-Zero integer is a positive integer that does not contain any 0 in its
// decimal representation.
//
// Given an integer n, return a list of two integers [a, b] where:
//
//
// a and b are No-Zero integers.
// a + b = n
//
//
// The test cases are generated so that there is at least one valid solution.
// If there are many valid solutions, you can return any of them.
//
//
// Example 1:
//
//
// Input: n = 2
// Output: [1,1]
// Explanation: Let a = 1 and b = 1.
// Both a and b are no-zero integers, and a + b = 2 = n.
//
//
// Example 2:
//
//
// Input: n = 11
// Output: [2,9]
// Explanation: Let a = 2 and b = 9.
// Both a and b are no-zero integers, and a + b = 11 = n.
// Note that there are other valid answers as [8, 3] that can be accepted.
//
//
//
// Constraints:
//
//
// 2 <= n <= 10^4
//
//

// @lc code=start
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn has_zero(e: i32) -> bool { return e.to_string().chars().into_iter().any(|e| e == '0') }
        for a in 1..n {
            let b = n - a;
            if has_zero(a) || has_zero(b) {
                continue;
            }
            return [a, b].to_vec();
        }
        unreachable!()
    }
}
// @lc code=end
