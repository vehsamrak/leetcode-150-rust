pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl Default for MinStack {
    fn default() -> Self {
        MinStack::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        match self.stack.last() {
            Some(x) => self
                .stack
                .push((val, val.min(x.1))),
            None => self.stack.push((val, val)),
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        match self.stack.last() {
            Some(x) => x.0,
            None => 0,
        }
    }

    pub fn get_min(&self) -> i32 {
        match self.stack.last() {
            Some(x) => x.1,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);

        assert_eq!(dbg!(min_stack.get_min()), -3);
        min_stack.pop();
        assert_eq!(dbg!(min_stack.top()), 0);
        assert_eq!(dbg!(min_stack.get_min()), -2);
    }
}
