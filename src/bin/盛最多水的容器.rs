/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_size = 0;
        while i < j {
            let curr_size = (j - i) as i32 * height[i].min(height[j]);
            max_size = max_size.max(curr_size);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_size
    }
}
// @lc code=end
