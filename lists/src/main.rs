use lists::BaseLinkedList;

fn main() {
    let mut list = BaseLinkedList::<u32>::new();

    list.push_back(10);
    list.push_back(15);

    dbg!(&list);
}
