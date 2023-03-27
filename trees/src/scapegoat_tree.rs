pub struct SGTree<T> {
    a: u32,
    b: u32,
    m: u32,
    n: u32,
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    key: u32,
    value: T,
    left_sub: Option<Box<SGTree<T>>>,
    right_sub: Option<Box<SGTree<T>>>,
    parent: Option<Box<SGTree<T>>>, // How do raw pointer to parent? This needs to be put on hold xd
}