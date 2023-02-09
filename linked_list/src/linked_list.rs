struct List<T> {
    head: &Node<T>,
    size: u32
}

struct Node<T> {
    data: <T>,
    next: Some(Node<T>)
}

impl List<T> {

}