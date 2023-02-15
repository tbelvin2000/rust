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

