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
impl Node {
    // Find successor to self
    // Returns: Pointer to successor Node or None
    fn successor(&mut self) -> Option<&mut Box<Node>> {
        let mut succ;
        let mut mv: u32 = 0;
        let mut two_child = false;
        match (matches!(self.left_sub.root, None), matches!(self.right_sub.root, None)) {
            // Self is leaf node
            (true, true) => return None,
            // Self has left descendents only
            (false, true) => return self.left_sub.root.as_mut(),
            // Self has right descendents only
            (true, false) => return self.right_sub.root.as_mut(),
            // Self has both descendents
            (false, false) => {
                succ = self.right_sub.root.as_mut().unwrap();
                let mut mv: u32 = 0;
                two_child = true;
                while !matches!(succ.left_sub.root, None) {
                    succ = succ.left_sub.root.as_mut().unwrap();
                    mv += 1;
                }
            },
        }
        // How to get inorder successor??
        if two_child { 
            let left_sub = self.left_sub.root.unwrap();
            succ.left_sub = Tree{root: Some(left_sub)};
            if mv != 0 {
                succ.right_sub = self.right_sub;
            }
        }
        Some(succ)
    }
}

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
    fn delete(&mut self, key: u32) -> bool {
        match &mut self.root {
            None => false,
            Some(rt) => match key.cmp(&rt.key) {
                Ordering::Equal => match rt.successor() {
                    None => {
                        self.root = None;
                        true
                    }
                    Some(succ) => {
                        rt = succ;
                        true
                    }
                },
                Ordering::Less => rt.left_sub.delete(key),
                Ordering::Greater => rt.right_sub.delete(key),
            },
        }
    }
}
