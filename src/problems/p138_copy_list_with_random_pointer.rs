use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if head.is_none() {
            return head;
        }

        let mut map: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();

        let mut current = head.clone();
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            let copied = Rc::new(RefCell::new(Node::new(node.val)));
            map.insert(Rc::as_ptr(&node_rc), copied);
            current = node.next.clone();
        }

        let mut current = head.clone();
        while let Some(node_rc) = current {
            let node = node_rc.borrow();

            let node_copied = map
                .get(&Rc::as_ptr(&node_rc))
                .unwrap()
                .clone();

            if let Some(next) = &node.next {
                let next_copied = map
                    .get(&Rc::as_ptr(next))
                    .unwrap()
                    .clone();
                node_copied.borrow_mut().next = Some(next_copied);
            }
            if let Some(random) = &node.random {
                let random_copied = map
                    .get(&Rc::as_ptr(random))
                    .unwrap()
                    .clone();
                node_copied
                    .borrow_mut()
                    .random = Some(random_copied);
            }

            current = node.next.clone();
        }

        map.get(&Rc::as_ptr(&head.unwrap()))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list_from_vec(nodes: Vec<(i32, Option<usize>)>) -> Option<Rc<RefCell<Node>>> {
        if nodes.is_empty() {
            return None;
        }

        let list_nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|(val, _)| Rc::new(RefCell::new(Node::new(*val))))
            .collect();

        for i in 0..nodes.len() {
            if i + 1 < nodes.len() {
                list_nodes[i]
                    .borrow_mut()
                    .next = Some(list_nodes[i + 1].clone());
            }
            if let Some(random_index) = nodes[i].1 {
                list_nodes[i]
                    .borrow_mut()
                    .random = Some(list_nodes[random_index].clone());
            }
        }

        Some(list_nodes[0].clone())
    }

    fn list_to_vec(head: Option<Rc<RefCell<Node>>>) -> Vec<(i32, Option<usize>)> {
        let mut result = Vec::new();
        if head.is_none() {
            return result;
        }

        let mut node_map = HashMap::new();
        let mut current = head.clone();
        let mut index = 0;
        while let Some(node) = current {
            node_map.insert(node.as_ptr() as usize, index);
            current = node.borrow().next.clone();
            index += 1;
        }

        let mut current = head;
        while let Some(node) = current {
            let node_ref = node.borrow();
            let random_index = node_ref
                .random
                .as_ref()
                .map(|random_node| node_map[&(random_node.as_ptr() as usize)]);
            result.push((node_ref.val, random_index));
            current = node_ref.next.clone();
        }

        result
    }

    #[test]
    fn test_copy_random_list() {
        let test_cases = [
            (
                vec![
                    (7, None),
                    (13, Some(0)),
                    (11, Some(4)),
                    (10, Some(2)),
                    (1, Some(0)),
                ],
                vec![
                    (7, None),
                    (13, Some(0)),
                    (11, Some(4)),
                    (10, Some(2)),
                    (1, Some(0)),
                ],
            ),
            (
                vec![(1, Some(1)), (2, Some(1))],
                vec![(1, Some(1)), (2, Some(1))],
            ),
            (
                vec![(3, None), (3, Some(0)), (3, None)],
                vec![(3, None), (3, Some(0)), (3, None)],
            ),
            (vec![], vec![]),
        ];

        for test_case in test_cases {
            let (input_vec, expected_vec) = dbg!(test_case);
            let head = build_list_from_vec(input_vec.to_vec());
            let copied_head = Solution::copy_random_list(head);
            let result_vec = list_to_vec(copied_head);

            assert_eq!(result_vec, expected_vec.to_vec());
        }
    }
}
