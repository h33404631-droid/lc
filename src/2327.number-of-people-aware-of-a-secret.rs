use super::prelude::*;

// @lc app=leetcode id=2327 lang=rust
//
// [2327] Number of People Aware of a Secret
//
// https://leetcode.com/problems/number-of-people-aware-of-a-secret/description/
//
// algorithms
// Medium (46.53%)
// Likes:    1255
// Dislikes: 150
// Total Accepted:    77.7K
// Total Submissions: 134K
// Testcase Example:  '6\n2\n4'
//
// On day 1, one person discovers a secret.
//
// You are given an integer delay, which means that each person will share the
// secret with a new person every day, starting from delay days after
// discovering the secret. You are also given an integer forget, which means
// that each person will forget the secret forget days after discovering it. A
// person cannot share the secret on the same day they forgot it, or on any day
// afterwards.
//
// Given an integer n, return the number of people who know the secret at the
// end of day n. Since the answer may be very large, return it modulo 10^9 +
// 7.
//
//
// Example 1:
//
//
// Input: n = 6, delay = 2, forget = 4
// Output: 5
// Explanation:
// Day 1: Suppose the first person is named A. (1 person)
// Day 2: A is the only person who knows the secret. (1 person)
// Day 3: A shares the secret with a new person, B. (2 people)
// Day 4: A shares the secret with a new person, C. (3 people)
// Day 5: A forgets the secret, and B shares the secret with a new person, D.
// (3 people)
// Day 6: B shares the secret with E, and C shares the secret with F. (5
// people)
//
//
// Example 2:
//
//
// Input: n = 4, delay = 1, forget = 3
// Output: 6
// Explanation:
// Day 1: The first person is named A. (1 person)
// Day 2: A shares the secret with B. (2 people)
// Day 3: A and B share the secret with 2 new people, C and D. (4 people)
// Day 4: A forgets the secret. B, C, and D share the secret with 3 new people.
// (6 people)
//
//
//
// Constraints:
//
//
// 2 <= n <= 1000
// 1 <= delay < forget <= n
//
//

// @lc code=start
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let delay = delay as usize;
        let forget = forget as usize;
        let n = n as usize;
        let mut dp = vec![0i32; n + 1];
        dp[1] = 1;
        let modulus = 1e9 as i32 + 7;
        let mut sharing_count = 0;
        let mut total_aware = 1;
        for i in 2..=n {
            // People who learned the secret `delay` days ago start sharing.
            if i > delay {
                sharing_count = (sharing_count + dp[i - delay]) % modulus;
            }

            // People who learned the secret `forget` days ago forget it.
            if i > forget {
                sharing_count = (sharing_count - dp[i - forget] + modulus) % modulus;
            }

            // The number of new people on day `i` is equal to the number of people
            // currently sharing.
            dp[i] = sharing_count;
            // Update the total number of people aware, ensuring the index is valid
            total_aware = (total_aware + dp[i]) % modulus;
            if i > forget {
                total_aware = (total_aware - dp[i - forget] + modulus) % modulus;
            }
        }
        total_aware
    }
}
// @lc code=end
