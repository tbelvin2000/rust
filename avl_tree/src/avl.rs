use std::{cmp::Ordering, collections::VecDeque};

#[derive(Default)]
pub struct Tree {
    root: Option<Box<Node>>,
}
struct Node {
    key: u32,
    left_sub: Tree,
    right_sub: Tree,
}

impl Node {
    fn new(key: u32) -> Node {
        Node {
            key,
            left_sub: Tree::default(),
            right_sub: Tree::default(),
        }
    }
}

impl Tree {
    // Returns a new tree initiated with no root or Node of key
    pub fn new(root: Option<u32>) -> Tree {
        if let Some(key) = root {
            Tree {
                root: Some(Box::new(Node::new(key))),
            }
        } else {
            Tree::default()
        }
    }

    // Insert node with key into tree
    // Returns: true if successfully inserted, false if node with key exists
    pub fn insert(&mut self, key: u32) -> bool {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    key,
                    left_sub: Tree::default(),
                    right_sub: Tree::default(),
                }));
                true
            },
            Some(rt) => match key.cmp(&rt.key) {
                Ordering::Equal => false,
                Ordering::Less => rt.left_sub.insert(key),
                Ordering::Greater => rt.right_sub.insert(key),
            },
        }
    }

    // Delete node with key in tree
    // Returns: true if successfull, else false
    pub fn delete(&mut self, key: u32) -> bool {
        match self.root.is_some() {
            // Empty tree
            false => false,
            true => {
                let current = self;
                match key.cmp(&current.root.as_ref().unwrap().key) {
                    // Delete current node
                    Ordering::Equal => {
                        match (
                            current.root.as_ref().unwrap().left_sub.root.is_some(),
                            current.root.as_ref().unwrap().right_sub.root.is_some(),
                        ) {
                            // Current is leaf
                            (false, false) => current.root = None,
                            // Current has only left descendents
                            (true, false) => current.root = current.root.take().unwrap().left_sub.root,
                            // Current has only right descendents
                            (false, true) => current.root = current.root.take().unwrap().right_sub.root,
                            // Current has both descendents
                            (true, true) => 
                                current.root.as_mut().unwrap().key = current
                                    .root
                                    .as_mut()
                                    .unwrap()
                                    .right_sub
                                    .extract_min()
                                    .unwrap(),
                        }
                        true
                    },
                    // Target may be in left subtree
                    Ordering::Less => current.root.as_mut().unwrap().left_sub.delete(key),
                    // Target may be in right subtree
                    Ordering::Greater => current.root.as_mut().unwrap().right_sub.delete(key),
                }
            }
        }
    }

    // Find key in self
    // Returns Vec of nodes visited to find key
    // Returns empty Vec if key was not found
    pub fn search(&mut self, key: u32) -> Vec<u32> {
        // Loads visited nodes to vec parameter
        // Returns true if key is found else false
        fn search_recursive(current: &mut Tree, key: u32, vec: &mut Vec<u32>) -> bool {
            match current.root.is_some() {
                false => false,
                true => {
                    vec.push(current.root.as_ref().unwrap().key);
                    match key.cmp(&current.root.as_ref().unwrap().key) {
                        // Target is self.root
                        Ordering::Equal => true,
                        // Target may be in right subtree
                        Ordering::Greater => search_recursive(
                            &mut current.root.as_mut().unwrap().right_sub, 
                            key, 
                            vec
                        ),
                        // Target may be in left subtree
                        Ordering::Less => search_recursive(
                            &mut current.root.as_mut().unwrap().left_sub, 
                            key, 
                            vec
                        ),
                    }
                }
            }
        }
        let mut vec = Vec::new();
        match search_recursive(self, key, &mut vec) {
            false => Vec::new(),
            true => vec,
        }
    }

    // Return a vector of keys in pre-order
    pub fn pre_order(&mut self) -> Vec<u32> {
        let mut vec = Vec::new();
        if self.root.is_some() {
            vec.push(self.root.as_ref().unwrap().key);
            vec.append(&mut self.root.as_mut().unwrap().left_sub.pre_order());
            vec.append(&mut self.root.as_mut().unwrap().right_sub.pre_order());
        }
        vec
    }

    // Return a vector of keys in post-order
    pub fn post_order(&mut self) -> Vec<u32> {
        let mut vec = Vec::new();
        if self.root.is_some() {
            vec.append(&mut self.root.as_mut().unwrap().left_sub.post_order());
            vec.append(&mut self.root.as_mut().unwrap().right_sub.post_order());
            vec.push(self.root.as_ref().unwrap().key);
        }
        vec
    }

    // Return a vector of keys in-order
    pub fn in_order(&mut self) -> Vec<u32> {
        let mut vec = Vec::new();
        if self.root.is_some() {
            vec.append(&mut self.root.as_mut().unwrap().left_sub.in_order());
            vec.push(self.root.as_ref().unwrap().key);
            vec.append(&mut self.root.as_mut().unwrap().right_sub.in_order());
        }
        vec
    }

    // Return a vector of keys in the ordering a breadth first traversal (left to right)
    pub fn bft(&mut self) -> Vec<u32> {
        fn bft_rec(vec: &mut Vec<u32>, que: &mut VecDeque<&Node>) {
            if !que.is_empty() {
                let node = que.pop_front().unwrap();
                vec.push(node.key);
                if node.left_sub.root.is_some() {
                    que.push_back(node.left_sub.root.as_ref().unwrap());
                }
                if node.right_sub.root.is_some() {
                    que.push_back(node.right_sub.root.as_ref().unwrap());
                }
                bft_rec(vec, que)
            }
        }
        let mut vec = Vec::new();
        if self.root.is_some() {
            let mut que: VecDeque<&Node> = VecDeque::new();
            que.push_back(self.root.as_ref().unwrap());
            bft_rec(&mut vec, &mut que);
        }
        vec
    }

    // Find and extract the minimum value of a tree
    // Replaces minimum value node with its right child if necessary
    // Returns: Some<u32> or None on empty tree
    pub fn extract_min(&mut self) -> Option<u32> {
        let mut current = self;
        let mut ret = None;
        // Check if empty tree
        if current.root.is_some() {
            // While left subtree is not none, move current to left subtree
            while current.root.as_ref().unwrap().left_sub.root.is_some() {
                current = &mut current.root.as_mut().unwrap().left_sub;
            }
            // Save minimum key
            ret = Some(current.root.as_ref().unwrap().key);
            // Remove min node (use take to avoid double borrow)
            current.root = current.root.take().unwrap().right_sub.root;
        }
        ret
    }

    // Find and extract minimum value of a tree
    // Replaces minimum value node with its left child if necessary
    // Returns: Some<u32> or None on empty tree
    pub fn extract_max(&mut self) -> Option<u32> {
        let mut current = self;
        let mut ret = None;
        if current.root.is_some() {
            while current.root.as_ref().unwrap().right_sub.root.is_some() {
                current = &mut current.root.as_mut().unwrap().right_sub;
            }
            ret = Some(current.root.as_ref().unwrap().key);
            current.root = current.root.take().unwrap().left_sub.root;
        }
        ret
    }
}