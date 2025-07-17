use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef<T> = Rc<RefCell<TreeNode<T>>>;

#[allow(dead_code)]
#[derive(Debug)]
struct TreeNode<T> {
    data: String,
    children: Vec<TreeNodeRef<T>>,
}

#[allow(dead_code)]
impl<T> TreeNode<T> {
    fn new(data: &str) -> TreeNodeRef<T> {
        Rc::new(RefCell::new(TreeNode {
            data: data.to_string(),
            children: vec![],
        }))
    }

    fn add_child(parent: &TreeNodeRef<T>, child: TreeNodeRef<T>) {
        parent.borrow_mut().children.push(child);
    }
}

fn _main() {
    let root: Rc<RefCell<TreeNode<String>>> = TreeNode::new("CEO");

    let head_of_sales = TreeNode::new("Head of Sales");
    let head_of_marketing = TreeNode::new("Head of Marketing");
    let sales_executive = TreeNode::new("Sales Executive");
    let marketing_executive = TreeNode::new("Marketing Executive");

    TreeNode::add_child(&root, Rc::clone(&head_of_sales));
    TreeNode::add_child(&root, Rc::clone(&head_of_marketing));

    TreeNode::add_child(&head_of_sales, Rc::clone(&sales_executive));
    TreeNode::add_child(&head_of_marketing, Rc::clone(&marketing_executive));
}
