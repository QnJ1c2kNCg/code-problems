use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};

struct Node {
    children: RefCell<Vec<Rc<Node>>>,
    is_terminal: AtomicBool,
    content: u8,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "content: {}, children: {:?}",
            self.content as char,
            self.children.borrow()
        )
    }
}

impl Node {
    fn new(content: u8) -> Self {
        Self {
            children: RefCell::new(vec![]),
            content,
            is_terminal: AtomicBool::new(false),
        }
    }

    fn add_child(&self, child_content: u8) -> Rc<Node> {
        let new_node = Rc::new(Node::new(child_content));
        self.children.borrow_mut().push(new_node.clone());
        new_node
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn mark_terminal(&self) {
        self.is_terminal
            .store(true, std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug)]
struct Trie {
    root: Rc<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Rc::new(Node::new(0u8)),
        }
    }

    fn insert(&mut self, word: String) {
        self.insert_inner(word.as_bytes(), self.root.clone());
    }

    fn insert_inner(&mut self, word: &[u8], parent_node: Rc<Node>) {
        if word.is_empty() {
            // insert the terminal marker
            parent_node.mark_terminal();
            return;
        }

        let first_char = word[0];
        let mut already_exists = false;

        for child in &*parent_node.children.borrow() {
            // the node for this char already exists, move along
            if child.content == first_char {
                already_exists = true;
                // continue the insertion with the rest
                self.insert_inner(&word[1..], child.clone());
            }
        }
        if !already_exists {
            // insert the new node
            let new_node = parent_node.add_child(first_char);

            // continue the insertion with the rest
            self.insert_inner(&word[1..], new_node);
        }
    }

    fn search(&self, word: String) -> bool {
        self.search_inner(word.as_bytes(), self.root.clone())
    }

    fn search_inner(&self, prefix: &[u8], parent_node: Rc<Node>) -> bool {
        if prefix.is_empty() {
            return parent_node.is_terminal();
        }

        let current_char = prefix[0];

        for child in &*parent_node.children.borrow() {
            // the node for this char already exists, move along
            if child.content == current_char {
                return self.search_inner(&prefix[1..], child.clone());
            }
        }

        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.starts_with_inner(prefix.as_bytes(), self.root.clone())
    }

    fn starts_with_inner(&self, prefix: &[u8], parent_node: Rc<Node>) -> bool {
        if prefix.is_empty() {
            return true;
        }

        let current_char = prefix[0];

        for child in &*parent_node.children.borrow() {
            // the node for this char already exists, move along
            if child.content == current_char {
                return self.starts_with_inner(&prefix[1..], child.clone());
            }
        }

        false
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_owned());
    assert!(trie.search("apple".to_owned())); // return True
    assert!(!trie.search("app".to_owned())); // return False
    assert!(trie.starts_with("app".to_owned())); // return True
    trie.insert("app".to_owned());
    assert!(trie.search("app".to_owned())); // return True

    println!("{:?}", trie);
}
