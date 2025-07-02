use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link<T> = Option<Rc<RefCell<DoublyNode<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<DoublyNode<T>>>>;

#[allow(dead_code)]
struct DoublyNode<T> {
    value: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

#[allow(dead_code)]
struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: WeakLink<T>,
    size: usize,
}

#[allow(dead_code)]
impl<T> DoublyLinkedList<T> {
    pub fn new(&self) -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn insert_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            value,
            next: self.head.clone(),
            prev: None,
        }));

        match &self.head {
            None => self.tail = Some(Rc::downgrade(&new_node)),
            Some(old_head) => old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node)),
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn insert_end(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            value,
            next: None,
            prev: None,
        }));

        match &self.tail {
            None => self.head = Some(new_node.clone()),
            Some(tail) => {
                if let Some(tail_strong) = tail.upgrade() {
                    tail_strong.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(Rc::downgrade(&tail_strong))
                }
            }
        }

        self.tail = Some(Rc::downgrade(&new_node));
        self.size += 1;
    }
}
