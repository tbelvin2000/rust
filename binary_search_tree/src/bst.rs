struct Node {
    key: u32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}
// Lifetimes are saying that struct Node will have Option pointers to nodes with at maximum
//     the same lifetime as itself. Is there a way to specify they will have a lesser lifespan?

impl Default for Node {
    // Should only be used to with a passed key value
    fn default() -> Self {
        Node {
            key: 0,
            left_child: None,
            right_child: None,
        }
    }
}

impl Node {
    // Search tree rooted at self for target node
    // Return reference to Node or none
    fn find(&self, target: u32) -> Option<&Node> {
        match target {
            x if x == self.key => Some(self),
            x if x < self.key => match &self.left_child {
                None => None,
                Some(lc) => lc.find(target),
            },
            x if x > self.key => match &self.right_child {
                None => None,
                Some(rc) => rc.find(target),
            },
            _ => panic!("x is somehow not related to self.key"),
        }
    }
    // Insert node with key in tree rooted at self
    // Return inserted Node if inserted, None if key is already in tree
    fn insert(&mut self, key: u32) -> bool {
        match key {
            // Key already exists in tree
            x if x == self.key => false,
            x if x < self.key => match &mut self.left_child {
                // Key is left child of current node
                None => {
                    self.left_child = Some(Box::new(Node {
                        key,
                        ..Default::default()
                    }));
                    true
                }
                // Key is left descendent of current node
                Some(lc) => lc.insert(key),
            },
            x if x > self.key => match &mut self.right_child {
                // Key is right child of current node
                None => {
                    self.right_child = Some(Box::new(Node {
                        key,
                        ..Default::default()
                    }));
                    true
                }
                // Key is right descendent of current node
                Some(rc) => rc.insert(key),
            },
            _ => panic!("x is sommehow not related to self.key"),
        }
    }
    // Delete target node from tree rooted at self
    // Return true if deleted, false if key is not in tree
    fn delete(&mut self, target: u32) -> Node {
        fn successor(target: &mut Node) -> Option<Box<Node>> {
            match (target.left_child, target.right_child) {
                // Target is leaf
                (None, None) => None,
                // Target has only left child
                (lc, None) => lc,
                // Target has only right child
                (None, rc) => rc,
                // Target has both children; Find in order successor
                // Just use parent and look at child, that way there is only one pointer
                (lc, Some(rc)) => {
                    let mut parent = rc.as_mut();
                    match parent.left_child {
                        // Successor is targets right child
                        None => {
                            let succ = Some(Box::new(Node{key: parent.key, left_child: target.left_child, right_child: parent.right_child}));
                            parent.delete(parent.key);
                            return succ;
                        }
                        // Successor is leftmost descendent
                        Some(lc) => {
                            /* Find leftmost descendent then build and return successor */
                            loop {
                                parent = lc.as_mut();
                                match parent.left_child {
                                    
                                }
                            }
                        }
                    }
                    let key = succ.key;
                    let left_child = target.left_child;
                    let right_child = if target.right_child.unwrap().as_ref().key != key {
                        target.right_child
                    } else {
                        succ.right_child
                    };
                    parent.delete(key);
                    Some(Box::new(Node {
                        key,
                        left_child,
                        right_child,
                    }))
                }
            }
        }
        match target {
            x if x == self.key => successor(self),
        }
    }
}
