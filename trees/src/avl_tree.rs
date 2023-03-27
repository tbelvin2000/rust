use std::{cmp::Ordering, collections::VecDeque, default};
pub struct Tree <T>{
    root: Option<Box<Node<T>>>,
}
struct Node <T>{
    key: u32,
    value: T,
    balance: i32,
    left_sub: Tree <T>,
    right_sub: Tree <T>,
}

impl<T> Default for Tree<T> {
    fn default() -> Tree<T> {
        Tree {
            root: None
        }
    }
}

impl<T> Node <T>{
    fn new(key: u32, value: T) -> Node<T>{
        Node {
            key,
            value,
            balance: 0,
            left_sub: Tree::default(),
            right_sub: Tree::default(),
        }
    }
}

impl<T> Tree <T>{
    // Returns a new tree initiated with no root or Node of key
    pub fn new(root: Option<(u32, T)>) -> Tree<T> {
        if let Some((key, value)) = root {
            Tree {
                root: Some(Box::new(Node::new(key, value))),
            }
        } else {
            Tree::default()
        }
    }

    // TODO: Only need to mess new root and old root balance
    fn left_rotation(&mut self) {
        // Take current root node
        let mut current = self.root.take();
        // Take and hold right child's left child
        let hold = current.as_mut().unwrap()
                    .right_sub.root.as_mut().unwrap()
                    .left_sub.root.take();
        // Take right child to be new root
        let mut new_root = current.as_mut().unwrap().right_sub.root.take();
        // Set current root's right child to held node
        current.as_mut().unwrap().right_sub.root = hold;
        // Set new balance conditions for old root and new root
        match new_root.as_ref().unwrap().balance == 0 {
            // right sub tree was perfectly balanced
            true => {
                // new tree will be perfectly balanced
                current.as_mut().unwrap().balance = 0;
                new_root.as_mut().unwrap().balance = 0;
            },
            // right sub tree was left leaning
            false => {
                // Current root will be right leaning, new root will be left leaning
                current.as_mut().unwrap().balance = 1;
                new_root.as_mut().unwrap().balance = -1;
            }
        }
        // Set new root's right child to current node
        new_root.as_mut().unwrap().right_sub.root = current;
        // Set self.root to new root
        self.root = new_root;
    }

    fn right_rotation(&mut self) {
        // Take current root node
        let mut current = self.root.take();
        // Take and hold left child's right child
        let hold = current.as_mut().unwrap()
                    .left_sub.root.as_mut().unwrap()
                    .right_sub.root.take();
        // Take left child to be new root
        let mut new_root = current.as_mut().unwrap().left_sub.root.take();
        // Set current root nodes left child to held node
        current.as_mut().unwrap().left_sub.root = hold;
        // Set new balance conditions for old root and new root
        match new_root.as_ref().unwrap().balance == 0 {
            // left sub tree was perfectly balanced
            true => {
                // new tree will be perfectly balanced
                current.as_mut().unwrap().balance = 0;
                new_root.as_mut().unwrap().balance = 0;
            },
            // left sub tree was right leaning
            false => {
                // Current root will be left leaning, new root will be right leaning
                current.as_mut().unwrap().balance = -1;
                current.as_mut().unwrap().balance = 1;
            }
        }

        // Set new root's right child to current node
        new_root.as_mut().unwrap().right_sub.root = current;
        // Set self.root to new root
        self.root = new_root;
    }

    // Balances tree rooted at self
    // Returns true if tree was rotated else false
    fn balance(&mut self) -> bool {
        let mut bal = self.root.as_ref().unwrap().balance;
        match bal {
            // Tree is left heavy
            b if b < -1 => {
                // Since bal < -1, root must have a left subtree to unwrap
                bal = self.root.as_ref().unwrap().left_sub.root.as_ref().unwrap().balance;
                // Tree is left right heavy
                if bal > 0 {
                    self.root.as_mut().unwrap().left_sub.left_rotation();
                    self.right_rotation();
                // Tree is left left heavy (and possibly left right heavy)
                } else {
                    self.right_rotation();
                }
                true
            },
            b if b > 1 => {
                // Since bal > 1, root must have right subtree to unwrap
                bal = self.root.as_ref().unwrap().right_sub.root.as_ref().unwrap().balance;
                // Tree is right left heavy
                if bal < 0 {
                    self.root.as_mut().unwrap().right_sub.right_rotation();
                    self.left_rotation();
                // Tree is right right heavy (and possibly right left heavy)
                } else {
                    self.left_rotation();
                }
                true
            },
            _ => false,
        }
    }

    // Insert node with key into tree
    // Returns: true if successfully inserted, false if node with key exists
    pub fn insert(&mut self, key: u32, value: T) -> bool {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    key,
                    value,
                    balance: 0,
                    left_sub: Tree::default(),
                    right_sub: Tree::default(),
                }));
                true
            },
            Some(rt) => match key.cmp(&rt.key) {
                Ordering::Equal => false,
                // TODO: Check if balance changed and rotate is required
                Ordering::Less => if rt.left_sub.insert(key, value) {
                    rt.balance -= 1; 
                    // Rotate necessary?
                    true
                } else {
                    false
                },
                Ordering::Greater => if rt.right_sub.insert(key, value) {
                    rt.balance += 1;
                    // Rotate?
                    true
                } else {
                    false
                },
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
                    // TODO is it necessary to update balance?
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
                    Ordering::Less => if current.root.as_mut().unwrap().left_sub.delete(key) {
                        current.root.as_mut().unwrap().balance += 1;
                        // TODO Rotate?
                        true
                    } else {
                        false
                    },
                    // Target may be in right subtree
                    Ordering::Greater => if current.root.as_mut().unwrap().right_sub.delete(key) {
                        current.root.as_mut().unwrap().balance -= 1;
                        // TODO Rotate?
                        true
                    } else {
                        false
                    },
                }
            }
        }
    }

    // TODO: Add rotations

    // Find key in self
    // Returns Vec of nodes visited to find key
    // Returns empty Vec if key was not found
    pub fn search(&mut self, key: u32) -> Vec<u32> {
        // Loads visited nodes to vec parameter
        // Returns true if key is found else false
        fn search_recursive<T>(current: &mut Tree<T>, key: u32, vec: &mut Vec<u32>) -> bool {
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
        fn bft_rec<T>(vec: &mut Vec<u32>, que: &mut VecDeque<&Node<T>>) {
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
            let mut que: VecDeque<&Node<T>> = VecDeque::new();
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