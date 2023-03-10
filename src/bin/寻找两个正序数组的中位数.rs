/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
use std::f64::INFINITY;
fn getty(l: &Vec<i32>, index: usize) -> (f64, f64) {
    let left = match index {
        0 => -INFINITY,
        x => l[x - 1] as f64,
    };
    let right = match index {
        _ if l.len() == index => INFINITY,
        x => l[x] as f64,
    };
    (left, right)
}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2) = (nums1.len(), nums2.len());
        if n1 > n2 {
            // make first arg smaller
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let total = n1 + n2;
        let half = total >> 1;
        let (mut lo, mut hi) = (0, n1);
        let (mut c1, mut c2);

        loop {
            let l = lo + hi >> 1;
            c1 = getty(&nums1, l);
            c2 = getty(&nums2, half - l);

            if c1.0 > c2.1 {
                hi = l - 1;
            } else if c2.0 > c1.1 {
                lo = l + 1;
            } else {
                break;
            }
        }

        match total % 2 {
            0 => (c1.0.max(c2.0) + c1.1.min(c2.1)) / 2.0,
            _ => c1.1.min(c2.1),
        }
    }

    pub fn find_median_sorted_arrays1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mid = (nums1.len() + nums2.len()) / 2;
        let is_two = (nums1.len() + nums2.len()) % 2 == 0;

        let mut last = 0;
        let mut one_before = last;
        let mut n1: usize = 0;
        let mut n2: usize = 0;
        for _ in 0..=mid {
            one_before = last;
            if n1 == nums1.len() {
                last = nums2[n2];
                n2 += 1;
            } else if n2 == nums2.len() {
                last = nums1[n1];
                n1 += 1;
            } else if nums1[n1] < nums2[n2] {
                last = nums1[n1];
                n1 += 1;
            } else {
                last = nums2[n2];
                n2 += 1;
            }
        }

        if is_two {
            (one_before + last) as f64 / 2.0
        } else {
            last as f64
        }
    }
}
// @lc code=end

struct Solution;
