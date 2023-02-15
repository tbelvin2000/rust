use std::cmp::Ordering;

#[derive(Default)]
struct Tree {
    root: Option<Box<Node>>,
}
struct Node {
    key: u32,
    left_sub: Tree,
    right_sub: Tree,
}
// Lifetimes are saying that struct Node will have Option pointers to nodes with at maximum
//     the same lifetime as itself. Is there a way to specify they will have a lesser lifespan?

impl Tree {
    // Insert node with key into tree
    // Returns: true if successfully inserted, false if node with key exists
    fn insert(&mut self, key: u32) -> bool {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    key,
                    left_sub: Tree::default(),
                    right_sub: Tree::default(),
                }));
                true
            }
            Some(rt) => match key.cmp(&rt.key) {
                Ordering::Equal => false,
                Ordering::Less => rt.left_sub.insert(key),
                Ordering::Greater => rt.right_sub.insert(key),
            },
        }
    }
    
    // Delete node with key in tree
    // Returns: true if successfull, else false
    fn delete(&mut self, key: u32) -> bool {
        if self.root.is_some() {
            let current = self;
            match key.cmp(&current.root.as_ref().unwrap().key) {
                Ordering::Equal => {
                    match (current.root.as_ref().unwrap().left_sub.root.is_some(),current.root.as_ref().unwrap().right_sub.root.is_some(),) {
                        // Self is leaf node
                        (false, false) => {
                            current.root = None;
                            true
                        }
                        // Self has left descendents only
                        (true, false) => {
                            current.root = current.root.take().unwrap().left_sub.root;
                            true
                        }
                        // Self has right descendents only
                        (false, true) => {
                            current.root = current.root.take().unwrap().right_sub.root;
                            true
                        }
                        // Self has both descendents
                        (true, true) => {
                            let mut successor = &mut current.root.as_mut().unwrap().right_sub;
                            while successor.root.as_ref().unwrap().left_sub.root.is_some() {
                                successor = &mut successor.root.as_mut().unwrap().left_sub;
                            }
                            let suc_key = successor.root.as_ref().unwrap().key;
                            successor.root = successor.root.take().unwrap().right_sub.root;
                            current.root.as_mut().unwrap().key = suc_key;
                            true
                        }
                    }
                }
                Ordering::Less => current.root.as_mut().unwrap().left_sub.delete(key),
                Ordering::Greater => current.root.as_mut().unwrap().right_sub.delete(key),
            }
        } else {
            false
        }
        
    }
}
