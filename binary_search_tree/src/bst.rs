use std::cmp::Ordering;
struct Tree(Option<Box<Node>>);
struct Node {
    key: u32,
    left_sub: Tree,
    right_sub: Tree,
}
// Lifetimes are saying that struct Node will have Option pointers to nodes with at maximum
//     the same lifetime as itself. Is there a way to specify they will have a lesser lifespan?

impl Node {
    fn new(key: u32) -> Node {
        Node {
            key,
            left_sub: Tree(None),
            right_sub: Tree(None),
        }
    }
}

impl Tree {
    fn insert(&mut self, key: u32) -> bool {
        match self.0.is_some() {
            false => {
                self.0 = Some(Box::new(Node::new(key)));
                true
            }
            true => {
                let root = self.0.as_mut().unwrap();
                match key.cmp(&root.key) {
                    Ordering::Equal => false,
                    Ordering::Less => root.left_sub.insert(key),
                    Ordering::Greater => root.right_sub.insert(key),
                }
            }
        }
    }
    // Find and remove minimum Node of tree
    // replace with right child if necessary
    // Returns key value of min node or None for tree of height 0
    fn extract_min(&mut self) -> Option<u32> {
        match self.0.is_some() {
            false => None,
            true => {
                let mut current = self;
                while current.0.as_mut().unwrap().left_sub.0.is_some() {
                    current = &mut current.0.as_mut().unwrap().left_sub;
                }
                let ret = current.0.as_mut().unwrap().key;
                current.0 = current.0.take().unwrap().left_sub.0;
                Some(ret)
            }
        }
    }
    // Find and remove node with key
    // replace with successor if necessary
    // Returns: true on success, false on failure
    fn delete(&mut self, key: u32) -> bool {
        match self.0.is_some() {
            false => false,
            true => {
                let current = self;
                match key.cmp(&current.0.as_mut().unwrap().key) {
                    // Delete current
                    Ordering::Equal => {
                        match (current.0.as_mut().unwrap().left_sub.0.is_some(), current.0.as_mut().unwrap().right_sub.0.is_some()) {
                            // current is leaf
                            (false, false) => current.0 = None,
                            (true, false) => current.0 = current.0.take().unwrap().left_sub.0,
                            (false, true) => current.0 = current.0.take().unwrap().right_sub.0,
                            (true, true) => current.0.as_mut().unwrap().key = current.0.as_mut().unwrap().right_sub.extract_min().unwrap(),
                        }
                        true
                    }
                    // Target may be in left descendents
                    Ordering::Less => current.0.as_mut().unwrap().left_sub.delete(key),
                    // Target may be in right descendents
                    Ordering::Greater => current.0.as_mut().unwrap().right_sub.delete(key),
                }
            }
        }
    }
}
