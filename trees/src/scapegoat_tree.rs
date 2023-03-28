use std::{mem, ptr, cmp::Ordering};

pub struct SGTree<T> {
    a: u32,
    b: u32,
    m: u32,
    n: u32,
    root: Option<Node<T>>,
}

pub struct Node<T> {
    key: u32,
    value: T,
    left_sub: Option<Box<SGTree<T>>>,
    right_sub: Option<Box<SGTree<T>>>,
    parent: Option<*mut Node<T>>, 
}

impl<T> Node<T> {
    pub fn new(key: u32, value: T, parent: Option<* mut Node<T>>) -> Node<T> {
        Node {key, value, left_sub: None, right_sub: None, parent}
    }
}

impl<T> SGTree<T> {
    pub fn new(a: u32, b: u32, root: Option<(u32, T)>) -> SGTree<T> {
         SGTree { a, b, m: 0, n: 0, root: None }
    }

    pub fn insert(&mut self, key: u32, value: T) -> bool {
        match self.root.is_none() {
            true => {
                self.root = Some(Node::new(key, value, None));
                self.m += 1;
                self.n += 1;
                true
            },
            false => {
                match key.cmp(&self.root.as_ref().unwrap().key){
                    Ordering::Equal => false,
                    Ordering::Less => match self.root.as_ref().unwrap().left_sub.is_none() {
                        true => {
                            self.root.as_mut().unwrap().left_sub = Some(
                                Box::new(SGTree { 
                                    a: self.a, b: self.b, m: 1, n: 1, root: Some(
                                    Node::new(
                                        key, value, 
                                        Some(self.root.as_mut().unwrap())
                                    )
                                )
                            }
                            ));
                            self.m += 1;
                            self.n += 1;
                            true
                        },
                        false => {
                            self.root.as_mut().unwrap().left_sub.as_mut().unwrap().insert(key, value)
                        }
                    } 
                    Ordering::Greater => match self.root.as_ref().unwrap().right_sub.is_none() {
                        true => {
                            self.root.as_mut().unwrap().right_sub = Some(
                                Box::new(SGTree { 
                                    a: self.a, b: self.b, m: 1, n: 1, root: Some(
                                    Node::new(
                                        key, value, 
                                        Some(self.root.as_mut().unwrap())
                                    )
                                )
                            }
                            ));
                            self.m += 1;
                            self.n += 1;
                            true
                        },
                        false => {
                            self.root.as_mut().unwrap().right_sub.as_mut().unwrap().insert(key, value)
                        }
                    }
                }
            }
        }
    }

}