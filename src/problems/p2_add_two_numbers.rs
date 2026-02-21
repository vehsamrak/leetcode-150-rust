#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: ListNode = ListNode::new(0);
        let mut current_node = &mut result;
        let (mut left, mut right) = (left, right);
        let mut carry = 0;

        while left.is_some() || right.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(l) = left {
                sum += l.val;
                left = l.next;
            }
            if let Some(r) = right {
                sum += r.val;
                right = r.next;
            }

            let last_digit = sum % 10;
            carry = sum / 10;

            current_node.next = Some(Box::new(ListNode::new(last_digit)));
            current_node = current_node
                .next
                .as_mut()
                .unwrap();
        }

        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in v.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn test_add_two_numbers() {
        struct TestCase {
            l1: Vec<i32>,
            l2: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = vec![
            TestCase {
                l1: vec![2, 4, 3],
                l2: vec![5, 6, 4],
                expected: vec![7, 0, 8],
            },
            TestCase {
                l1: vec![0],
                l2: vec![0],
                expected: vec![0],
            },
            TestCase {
                l1: vec![9, 9, 9, 9, 9, 9, 9],
                l2: vec![9, 9, 9, 9],
                expected: vec![8, 9, 9, 9, 0, 0, 0, 1],
            },
        ];

        for test_case in test_cases {
            dbg!(&test_case.l1, &test_case.l2);
            let result = Solution::add_two_numbers(to_list(test_case.l1), to_list(test_case.l2));
            assert_eq!(to_list(test_case.expected), result);
        }
    }
}
