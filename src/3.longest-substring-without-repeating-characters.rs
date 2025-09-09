use super::prelude::*;

// @lc app=leetcode id=3 lang=rust
//
// [3] Longest Substring Without Repeating Characters
//
// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
//
// algorithms
// Medium (37.49%)
// Likes:    43014
// Dislikes: 2105
// Total Accepted:    8.1M
// Total Submissions: 21.6M
// Testcase Example:  '"abcabcbb"'
//
// Given a string s, find the length of the longest substring without duplicate
// characters.
//
//
// Example 1:
//
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
//
//
// Example 2:
//
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
//
//
// Example 3:
//
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not
// a substring.
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 5 * 10^4
// s consists of English letters, digits, symbols and spaces.
//
//

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::{cmp::max, collections::HashMap};

        if s.is_empty() {
            return 0;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut char_index_map: HashMap<char, usize> = HashMap::new();
        let mut max_length = 0;
        let mut start = 0;

        for (end, &ch) in chars.iter().enumerate() {
            if let Some(&last_index) = char_index_map.get(&ch) {
                if last_index >= start {
                    start = last_index + 1;
                }
            }

            char_index_map.insert(ch, end);
            max_length = max(max_length, end - start + 1);
        }

        max_length as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("pwwkew", 3)]
    #[case("abcabcbb", 3)]
    fn test(#[case] input: String, #[case] want: i32) {
        let res = Solution::length_of_longest_substring(input.clone());
        assert_eq!(res, want);
    }
}
