struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::default(),
        }
    }

    fn push(&mut self, val: i32) {
        // check the current min
        let min_val = if let Some(current_min) = self.get_min_inner() {
            if val < current_min { val } else { current_min }
        } else {
            val
        };
        self.stack.push((val, min_val));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack
            .last()
            .expect("the problem guarantees that `top` is called on a non-empty stack")
            .0
    }

    fn get_min(&self) -> i32 {
        self.stack
            .last()
            .expect("the problem guarantees that `get_min` is called on a non-empty stack")
            .1
    }

    fn get_min_inner(&self) -> Option<i32> {
        self.stack.last().map(|e| e.1)
    }
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(stack.get_min(), -3);
    stack.pop();
    assert_eq!(stack.top(), 0);
    assert_eq!(stack.get_min(), -2);
}
