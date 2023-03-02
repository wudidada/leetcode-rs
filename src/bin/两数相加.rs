/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

use std::{borrow::BorrowMut, mem::swap};

use leetcode_rs::ListNode;
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
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;
        let mut carry = false;

        let mut num1 = &l1;
        let mut num2 = &l2;

        loop {
            let mut total = 0;

            match (num1, num2) {
                (Some(n1), Some(n2)) => {
                    total = n1.val + n2.val + carry as i32;
                    num1 = &n1.next;
                    num2 = &n2.next;
                }
                (Some(n1), None) => {
                    total = n1.val + carry as i32;
                    num1 = &n1.next;
                }
                (None, Some(n2)) => {
                    total = n2.val + carry as i32;
                    num2 = &n2.next;
                }
                (None, None) => {
                    break;
                }
            }

            if total >= 10 {
                total %= 10;
                carry = true;
            } else {
                carry = false;
            }

            current.next = Some(Box::new(ListNode::new(total)));
            current = current.next.as_mut().unwrap();
        }
        if carry {
            current.next = Some(Box::new(ListNode::new(1)));
        }
        front.next
    }

    pub fn add_two_numbers1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers1(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers1(
                            Solution::add_two_numbers1(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}
// @lc code=end

struct Solution;
