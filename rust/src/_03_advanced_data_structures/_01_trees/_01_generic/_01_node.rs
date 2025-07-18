use std::cell::RefCell;
use std::rc::Rc;

pub type GenericTreeNodeRef<T> = Rc<RefCell<GenericTreeNode<T>>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GenericTreeNode<T> {
    pub data: String,
    pub children: Vec<GenericTreeNodeRef<T>>,
}

#[allow(dead_code)]
impl<T> GenericTreeNode<T> {
    fn new(data: &str) -> GenericTreeNodeRef<T> {
        Rc::new(RefCell::new(GenericTreeNode {
            data: data.to_string(),
            children: vec![],
        }))
    }

    fn add_child(parent: &GenericTreeNodeRef<T>, child: GenericTreeNodeRef<T>) {
        parent.borrow_mut().children.push(child);
    }
}

fn _main() {
    let root: Rc<RefCell<GenericTreeNode<String>>> = GenericTreeNode::new("CEO");

    let head_of_sales = GenericTreeNode::new("Head of Sales");
    let head_of_marketing = GenericTreeNode::new("Head of Marketing");
    let sales_executive = GenericTreeNode::new("Sales Executive");
    let marketing_executive = GenericTreeNode::new("Marketing Executive");

    GenericTreeNode::add_child(&root, Rc::clone(&head_of_sales));
    GenericTreeNode::add_child(&root, Rc::clone(&head_of_marketing));

    GenericTreeNode::add_child(&head_of_sales, Rc::clone(&sales_executive));
    GenericTreeNode::add_child(&head_of_marketing, Rc::clone(&marketing_executive));
}
