use std::io::Empty;

use data_structures::linked_list::{Link, List, Node};
fn main() {
    let mut list = List::new();
    let head = Node {
        elem: 1,
        next: Link::Empty,
    };
    let second = Node {
        elem: 2,
        next: Link::More(Box::new(head)),
    };
    let link = Link::More(Box::new(head));
    list.head = link;
}
