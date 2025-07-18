use core::cell::RefCell;
use std::rc::Rc;

use super::_01_node::GenericTreeNode;

pub fn _count_nodes<T>(root: Option<Rc<RefCell<GenericTreeNode<T>>>>) -> usize {
    match root {
        None => 0,
        Some(node) => {
            let node_borrowed = node.borrow();
            let mut count = 1;
            for child in &node_borrowed.children {
                count += _count_nodes(Some(child.clone()));
            }
            count
        }
    }
}
