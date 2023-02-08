struct Node<'a> {
    key: u32,
    left_child: Option<&'a Node<'a>>,
    right_child: Option<&'a Node<'a>>,
}
// Lifetimes are saying that struct Node will have Option pointers to nodes with at maximum
//     the same lifetime as itself. Is there a way to specify they will have a lesser lifespan?

impl<'a> Node<'a> {
    // Search tree rooted at self for target node
    // Return reference to Node or none
    fn find(&self, target: u32) -> Option<&Node> {
        match target {
            x if x == self.key => Some(&self),
            x if x < self.key => match self.left_child {
                None => None,
                Some(lc) => lc.find(target),
            },
            x if x > self.key => match self.right_child {
                None => None,
                Some(rc) => rc.find(target),
            }
            _ => panic!("x is somehow not related to self.key")
        }
    }
    // Insert node with key in tree rooted at self
    // Return true if inserted, false if key is already in tree
    fn insert(&mut self, key: u32) -> bool {
        todo!("Insert method")
    }
    // Delete target node from tree rooted at self
    // Return true if deleted, false if key is not in tree
    fn delete(&mut self, target: u32) -> bool {
        todo!("Delete method")
    }
}
