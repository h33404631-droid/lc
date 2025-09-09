use super::prelude::*;

// @lc app=leetcode id=1304 lang=rust
//
// [1304] Find N Unique Integers Sum up to Zero
//
// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/description/
//
// algorithms
// Easy (76.19%)
// Likes:    2216
// Dislikes: 607
// Total Accepted:    287.7K
// Total Submissions: 372.8K
// Testcase Example:  '5'
//
// Given an integer n, return any array containing n unique integers such that
// they add up to 0.
//
//
// Example 1:
//
//
// Input: n = 5
// Output: [-7,-1,1,3,4]
// Explanation: These arrays also are accepted [-5,-1,1,2,3] ,
// [-3,-1,2,-2,4].
//
//
// Example 2:
//
//
// Input: n = 3
// Output: [-1,0,1]
//
//
// Example 3:
//
//
// Input: n = 1
// Output: [0]
//
//
//
// Constraints:
//
//
// 1 <= n <= 1000
//
//

// @lc code=start
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(n as usize);
        for i in 1..=n / 2 {
            res.push(i);
            res.push(-i);
        }
        if n % 2 != 0 {
            res.push(0);
        }
        res
    }
}
// @lc code=end
