struct MyCircularQueue<T> {
    inner: Vec<Option<T>>,
    /// points to the first value of the queue
    head: usize,
    /// points to where the next value should be inserted
    next: usize,
    /// size of the buffer
    len: usize,
}

impl<T> MyCircularQueue<T> {
    fn new(k: i32) -> Self {
        Self {
            inner: std::iter::repeat_with(|| None).take(k as usize).collect(),
            head: 0,
            next: 0,
            len: 0,
        }
    }

    fn en_queue(&mut self, value: T) -> bool {
        if self.is_full() {
            return false;
        }

        *self
            .inner
            .get_mut(self.next)
            .expect("access out of bound, should never happen!") = Some(value);

        self.next = self.next_index(self.next);
        self.len += 1;

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        // *self
        //     .inner
        //     .get_mut(self.head)
        //     .expect("access out of bound, should never happen!") = value;

        self.head = self.next_index(self.head);
        self.len -= 1;

        true
    }

    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.inner
            .get(self.head)
            .expect("access out of bound, should never happen!")
            .as_ref()
    }

    fn rear(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.inner
            .get(self.previous_index(self.next))
            .expect("access out of bound, should never happen!")
            .as_ref()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.inner.capacity()
    }

    /// returns the next index
    /// This function handles the wrap around at the end of the buffer
    fn next_index(&self, index: usize) -> usize {
        // If the index is at the end of the buffer, then we
        // start back from the beginning
        if index == self.inner.capacity() - 1 {
            0
        } else {
            index + 1
        }
    }

    /// returns the previous index
    /// This function handles the wrap around at the start of the buffer
    fn previous_index(&self, index: usize) -> usize {
        // If the index is at the end of the buffer, then we
        // start back from the beginning
        if index == 0 {
            self.inner.capacity() - 1
        } else {
            index - 1
        }
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(128);
    let ret_1: bool = obj.en_queue(42);
    let ret_3 = *obj.front().unwrap();
    let ret_4 = *obj.rear().unwrap();
    let ret_2: bool = obj.de_queue();
    let ret_5: bool = obj.is_empty();
    let ret_6: bool = obj.is_full();
    println!("{}", ret_1);
    println!("{}", ret_2);
    println!("{}", ret_3);
    println!("{}", ret_4);
    println!("{}", ret_5);
    println!("{}", ret_6);
}
