/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::{
    cmp::{max, min},
    collections::HashMap,
};
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut curr_len = 0;
        let mut char_map = HashMap::new();

        for (i, c) in s.chars().into_iter().enumerate() {
            match char_map.insert(c, i) {
                Some(pre_i) => {
                    curr_len = min(curr_len + 1, i - pre_i);
                    max_len = max(max_len, curr_len);
                }
                None => {
                    curr_len += 1;
                    max_len = max(max_len, curr_len);
                }
            }
        }
        max_len as i32
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    let s = "dvdf".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);

    let s = "abba".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);

    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);

    let s = "aab".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);

    let s = "bbbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
}
