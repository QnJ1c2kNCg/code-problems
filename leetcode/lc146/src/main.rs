use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LRUCache {
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    capacity: usize,
    head: Option<usize>,
    tail: Option<usize>,
    /// indexes that have been evicted
    free_list: Vec<usize>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // check if the node key already exists
        match self.map.get(&key) {
            Some(index) => {
                // The key already exists, we need to update the value and make it the freshest
                self.nodes.get_mut(*index).unwrap().value = value;
                self.move_to_front(*index)
            }
            None => {
                if self.map.len() >= self.capacity {
                    self.remove_lru();
                }
                self.add_node(key, value);
            }
        }
        todo!()
    }

    fn add_node(&mut self, key: i32, value: i32) {
        let node = Node {
            key,
            value,
            next: self.head,
            prev: None,
        };
        let new_index = if let Some(index) = self.free_list.pop() {
            self.nodes[index] = node;
            index
        } else {
            self.nodes.push(node);
            self.nodes.len() - 1
        };

        // update the previous head
        if let Some(head_index) = self.head {
            self.nodes[head_index].prev = Some(new_index);
        }
        // update head
        self.head = Some(new_index);

        // in case there was never a tail set
        if self.tail.is_none() {
            self.tail = Some(new_index);
        }

        self.map.insert(key, new_index);
    }

    fn move_to_front(&mut self, idx: usize) {
        // update the current head node
        if let Some(head_index) = self.head {
            self.nodes[head_index].prev = Some(idx);
            self.nodes[idx].next = Some(head_index);
        }
        self.head = Some(idx);
    }

    fn remove_lru(&mut self) {
        if let Some(tail_index) = self.tail {
            let node = &self.nodes[tail_index];
            self.map.remove(&node.key).unwrap();
            self.free_list.push(tail_index);

            let prev = node.prev;
            let next = node.next;
            if let Some(prev_index) = prev {
                self.nodes[prev_index].next = None;
                self.tail = prev_index
            } else {
                // cache is empty
                self.head = None;
                self.tail = None;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
