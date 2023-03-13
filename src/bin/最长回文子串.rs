/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..bytes.len() {
            let (res_i, res_j) = longest(bytes, i, i);

            let curr_len = res_j - res_i + 1;

            if curr_len > max_len {
                max_len = curr_len;
                start = res_i;
            }
        }

        for i in 0..bytes.len() - 1 {
            let (res_i, res_j) = longest(bytes, i, i + 1);
            let curr_len = res_j - res_i + 1;
            if curr_len > max_len {
                max_len = curr_len;
                start = res_i;
            }
        }
        String::from_utf8(bytes[start..start + max_len].to_vec()).unwrap()
    }
}

fn longest(bytes: &[u8], mut i: usize, mut j: usize) -> (usize, usize) {
    loop {
        if bytes[i] != bytes[j] {
            return (i + 1, j - 1);
        }
        if i == 0 || j == bytes.len() - 1 {
            return (i, j);
        }
        i -= 1;
        j += 1;
    }
}

// @lc code=end

struct Solution;

#[test]
fn test() {
    let s = "babad".to_string();
    assert_eq!(Solution::longest_palindrome(s), "bab".to_owned());
}
