use crate::util::singly_linked_list::ListNode;

/// You are given two non-empty linked lists representing two non-negative
/// integers. The most significant digit comes first and each of their nodes
/// contain a single digit. Add the two numbers and return it as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the
/// number 0 itself.
///
/// Follow up:
/// What if you cannot modify the input lists? In other words, reversing the
/// lists is not allowed.
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Safe to unwrap because linked lists are guaranteed non-empty
        let l1_len = l1.as_ref().unwrap().len();
        let l2_len = l2.as_ref().unwrap().len();
        let (mut sum, carry) = Solution::recursive_add(l1, l2, l1_len, l2_len);
        let sum_plus_carry = sum.as_ref().unwrap().val + carry;
        if sum_plus_carry <= 9 {
            sum.as_mut().unwrap().val += carry;
            sum
        } else {
            let mut sum_head = ListNode::new(1);
            sum_head.next = sum;
            Some(Box::new(sum_head))
        }
    }

    fn recursive_add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        l1_len: i32,
        l2_len: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match l1_len.cmp(&l2_len) {
            Less => {
                let l1_val = 0;
                let l2_val = l2.as_ref().unwrap().val;
                let (sum_of_lower_digits, carry_of_lower_digits) =
                    Solution::recursive_add(l1, l2.unwrap().next, l1_len, l2_len - 1);
                let (sum_of_current_digits, carry_of_current_digits) =
                    Solution::add_digits_and_carry(l1_val, l2_val, carry_of_lower_digits);
                let mut sum_head = ListNode::new(sum_of_current_digits);
                sum_head.next = sum_of_lower_digits;
                (Some(Box::new(sum_head)), carry_of_current_digits)
            }
            Greater => {
                let l1_val = l1.as_ref().unwrap().val;
                let l2_val = 0;
                let (sum_of_lower_digits, carry_of_lower_digits) =
                    Solution::recursive_add(l1.unwrap().next, l2, l1_len - 1, l2_len);
                let (sum_of_current_digits, carry_of_current_digits) =
                    Solution::add_digits_and_carry(l1_val, l2_val, carry_of_lower_digits);

                let mut sum_head = ListNode::new(sum_of_current_digits);
                sum_head.next = sum_of_lower_digits;
                (Some(Box::new(sum_head)), carry_of_current_digits)
            }
            Equal => {
                let l1_val = l1.as_ref().unwrap().val;
                let l2_val = l2.as_ref().unwrap().val;
                // If this is the least significant digit
                if l1_len == 1 {
                    let (sum_of_current_digits, carry_of_current_digits) =
                        Solution::add_digits_and_carry(l1_val, l2_val, 0);
                    (
                        Some(Box::new(ListNode::new(sum_of_current_digits))),
                        carry_of_current_digits,
                    )
                } else {
                    let (sum_of_lower_digits, carry_of_lower_digits) = Solution::recursive_add(
                        l1.unwrap().next,
                        l2.unwrap().next,
                        l1_len - 1,
                        l2_len - 1,
                    );
                    let (sum_of_current_digits, carry_of_current_digits) =
                        Solution::add_digits_and_carry(l1_val, l2_val, carry_of_lower_digits);
                    let mut sum_head = ListNode::new(sum_of_current_digits);
                    sum_head.next = sum_of_lower_digits;
                    (Some(Box::new(sum_head)), carry_of_current_digits)
                }
            }
        }
    }

    /// Add two digits plus a carry. Both digits must be less than or equal to 9
    /// Returns the resulting digit and new carry
    fn add_digits_and_carry(digit1: i32, digit2: i32, carry: i32) -> (i32, i32) {
        let sum = digit1 + digit2 + carry;
        let new_carry = sum / 10;
        let digit = if new_carry == 0 { sum } else { sum % 10 };
        (digit, new_carry)
    }
}

impl ListNode {
    fn len(&self) -> i32 {
        let mut count = 1;
        let mut next = &self.next;
        while let Some(node) = next {
            count += 1;
            next = &node.next;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::sll;
    use crate::util::singly_linked_list::to_singly_linked_list;

    use super::*;

    #[test]
    fn test_7_243_plus_564() {
        assert_eq!(
            sll![7, 8, 0, 7],
            Solution::add_two_numbers(sll![7, 2, 4, 3], sll![5, 6, 4])
        );
    }

    #[test]
    fn test_0_plus_0() {
        assert_eq!(sll![0], Solution::add_two_numbers(sll![0], sll![0]))
    }

    #[test]
    fn test_3_999_999_999_plus_7() {
        assert_eq!(
            sll![4, 0, 0, 0, 0, 0, 0, 0, 0, 6],
            Solution::add_two_numbers(sll![3, 9, 9, 9, 9, 9, 9, 9, 9, 9], sll![7])
        )
    }

    #[test]
    fn test_9_999_999_999_999_plus_0() {
        assert_eq!(
            sll![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            Solution::add_two_numbers(sll![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], sll![0])
        )
    }
}
