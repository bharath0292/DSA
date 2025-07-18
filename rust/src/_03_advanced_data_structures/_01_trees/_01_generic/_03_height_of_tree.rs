use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use super::_01_node::GenericTreeNode;

fn _height_of_tree<T>(root: Option<Rc<RefCell<GenericTreeNode<T>>>>) -> usize {
    match root {
        None => 0,
        Some(node) => {
            let node_ref = node.borrow();
            let mut height: usize = 1;
            let mut max_child_height = 0;

            for child in &node_ref.children {
                let child_height = _height_of_tree(Some(Rc::clone(child)));
                max_child_height = cmp::max(max_child_height, child_height);
            }

            height += max_child_height;

            height
        }
    }
}
