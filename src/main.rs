use data_structures::linked_list::List;

fn main() {
    let mut list = List::new();

    list.push(13);
    list.push(14);
    list.push(15);
    list.push(16);
    list.push(17);

    println!("{:#?}", list);

    list.pop();
}
