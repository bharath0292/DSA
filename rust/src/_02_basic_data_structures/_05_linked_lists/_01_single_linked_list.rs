use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[allow(dead_code)]
struct SinglyLinkedList<T> {
    head: Link<T>,
    size: usize,
}

#[allow(dead_code)]
impl<T> SinglyLinkedList<T>
where
    T: PartialEq,
{
    pub fn new(&self) -> Self {
        SinglyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn insert_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
        }));

        self.head = Some(new_node);
        self.size += 1
    }

    fn insert_end(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));

        match &self.head {
            None => self.head = Some(new_node),
            Some(head) => {
                let mut current = head.clone();
                loop {
                    let next = current.borrow().next.clone();

                    match next {
                        Some(next_node) => current = next_node,
                        None => {
                            current.borrow_mut().next = Some(new_node);
                            break;
                        }
                    }
                }
            }
        }
        self.size += 1;
    }

    pub fn delete(&mut self, value: T) -> bool {
        let mut current = match self.head.clone() {
            None => return false,
            Some(node) => node,
        };

        if current.borrow().value == value {
            self.head = current.borrow().next.clone();
            self.size -= 1;
            return true;
        }

        loop {
            let next_link = current.borrow().next.clone();
            match next_link {
                Some(next_node) => {
                    if next_node.borrow().value == value {
                        let next_next = next_node.borrow().next.clone();
                        current.borrow_mut().next = next_next;
                        self.size -= 1;
                        return true;
                    } else {
                        current = next_node;
                    }
                }
                None => break,
            }
        }

        false
    }

    fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.clone();

        while let Some(current_node) = current {
            let next = current_node.borrow_mut().next.clone();
            current_node.borrow_mut().next = prev;
            prev = Some(current_node);
            current = next
        }

        self.head = prev
    }
}
