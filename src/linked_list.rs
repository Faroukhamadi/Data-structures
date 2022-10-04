// pub struct Node<'a, T> {
//     pub value: T,
//     pub next: Option<Box<&'a Node<'a, T>>>,
// }

pub struct List {
    pub head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    pub elem: i32,
    pub next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
