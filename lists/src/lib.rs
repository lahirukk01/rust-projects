use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next_node: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next_node: RefCell::new(None),
        }
    }

    pub fn new_rc(value: T) -> Rc<Self> {
        Rc::new(Self::new(value))
    }
}

#[derive(Debug)]
pub struct BaseLinkedList<T> {
    size: u32,
    start: RefCell<Option<Rc<Node<T>>>>,
    end: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> BaseLinkedList<T> {
    pub fn new() -> Self {
        BaseLinkedList {
            size: 0,
            start: RefCell::new(None),
            end: RefCell::new(None),
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node_rc = Node::new_rc(value);
        let old_end_rc = self.end.borrow_mut().replace(new_node_rc.clone());

        match old_end_rc {
            Some(prev_end_node_rc) => {
                prev_end_node_rc.next_node.borrow_mut().replace(new_node_rc);
            }
            None => {
                self.start.borrow_mut().replace(new_node_rc);
            }
        }

        self.size += 1;
    }
}

// pub struct LinkedListIterator<'a, T> {
//     current: Option<&'a RefCell<Box<Node<T>>>>,
// }

// impl<'a, T> Iterator for LinkedListIterator<'a, T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item>
//     where
//         T: Copy,
//     {
//         self.current.map(|node_ref_cell| {
//             let node = node_ref_cell.borrow();
//             self.current = node.next_node.borrow().as_ref();
//             node.value
//         })
//     }
// }

// impl<T> BaseLinkedList<T> {
//     pub fn iter(&self) -> LinkedListIterator<T> {
//         LinkedListIterator {
//             current: self.start.as_ref(),
//         }
//     }
// }
