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
    pub fn merge_two_lists(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut tail = &mut result;
        while left.is_some() && right.is_some() {
            if left.as_ref()?.val < right.as_ref()?.val {
                let mut node = left.take()?;
                left = node.next.take();
                tail.next = Some(node);
            } else {
                let mut node = right.take()?;
                right = node.next.take();
                tail.next = Some(node);
            }

            tail = tail.next.as_mut()?;
        }

        if left.is_some() {
            tail.next = left;
        } else {
            tail.next = right;
        }

        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>> = None;
        for value in values.into_iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = current;
            current = Some(node);
        }
        current
    }

    fn list_to_vector(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn test() {
        let result = Solution::merge_two_lists(None, None);
        assert_eq!(list_to_vector(result), vec![]);

        let list_two = build_list(vec![1, 2, 4]);
        let result = Solution::merge_two_lists(None, list_two);
        assert_eq!(list_to_vector(result), vec![1, 2, 4]);

        let list_one = build_list(vec![1, 3, 5]);
        let result = Solution::merge_two_lists(list_one, None);
        assert_eq!(list_to_vector(result), vec![1, 3, 5]);

        let list_one = build_list(vec![1, 2, 4]);
        let list_two = build_list(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(list_one, list_two);
        assert_eq!(list_to_vector(result), vec![1, 1, 2, 3, 4, 4]);

        let list_one = build_list(vec![-3, -1, 2, 2]);
        let list_two = build_list(vec![-2, 2, 3]);
        let result = Solution::merge_two_lists(list_one, list_two);
        assert_eq!(list_to_vector(result), vec![-3, -2, -1, 2, 2, 2, 3]);

        let list_one = build_list(vec![1]);
        let list_two = build_list(vec![0]);
        let result = Solution::merge_two_lists(list_one, list_two);
        assert_eq!(list_to_vector(result), vec![0, 1]);
    }
}
