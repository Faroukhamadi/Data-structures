use std::collections;

use data_structures::linked_list::Node;

collections::linked_list
pub mod garden;

fn main() {
    let second = Node {
        value: 2,
        next: None,
    };
    let head = Node {
        value: 3,
        next: Some(Box::new(&second)),
    };

    let mut cur = &head;
    while !cur.next.is_none() {
        cur = cur.next;
    }

    // match head.next {
    //     Some(_) => println!("wohoo have next"),
    //     None => println!("We sadly don't have next"),
    // }
}
