use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    numbers: Vec<i32>,
    index: HashMap<i32, usize>,
}

#[allow(dead_code)]
impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            return false;
        }

        self.numbers.push(val);
        self.index
            .insert(val, self.numbers.len() - 1);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        let deleting_item_index = match self.index.remove(&val) {
            Some(item) => item,
            None => return false,
        };

        let last_item_index = self.numbers.len() - 1;
        if deleting_item_index == last_item_index {
            self.numbers.pop();
            return true;
        }

        self.index
            .insert(self.numbers[last_item_index], deleting_item_index);
        self.numbers
            .swap_remove(deleting_item_index);

        true
    }

    fn get_random(&self) -> i32 {
        *self
            .numbers
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = RandomizedSet::new();

        let result: bool = obj.insert(1);
        assert!(result);
        let result: bool = obj.insert(1);
        assert!(!result);
        let result: i32 = obj.get_random();
        assert_eq!(1, result);
        let result: bool = obj.insert(2);
        assert!(result);
        let result: bool = obj.remove(1);
        assert!(result);
        let result: i32 = obj.get_random();
        assert_eq!(2, result);
        let result: bool = obj.remove(2);
        assert!(result);
        let result: bool = obj.insert(2);
        assert!(result);
    }
}
