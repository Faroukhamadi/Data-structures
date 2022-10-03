pub struct Node<'a, T> {
    pub value: T,
    pub next: Option<Box<&'a Node<'a, T>>>,
}
