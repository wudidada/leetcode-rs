/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

use core::num;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(prev) = map.get(&(target - *num)) {
                return vec![*prev, i as i32];
            } else {
                map.insert(*num, i as i32);
            }
        }
        panic!()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let res = Solution::two_sum(nums, target);
    assert_eq!(res[0], 1);
    assert_eq!(res[1], 2);
}
