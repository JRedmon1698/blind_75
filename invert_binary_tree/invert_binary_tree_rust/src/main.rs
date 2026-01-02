use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let node_9 = TreeNode::new(9);
    let node_6 = TreeNode::new(6);
    let node_3 = TreeNode::new(3);
    let node_1 = TreeNode::new(1);
    let mut node_7 = TreeNode::new(7);
    node_7.left = Some(Rc::new(RefCell::new(node_6)));
    node_7.right = Some(Rc::new(RefCell::new(node_9)));
    let mut node_2 = TreeNode::new(2);
    node_2.left = Some(Rc::new(RefCell::new(node_1)));
    node_2.right = Some(Rc::new(RefCell::new(node_3)));
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(node_2)));
    root.right = Some(Rc::new(RefCell::new(node_7)));
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());

    while let Some(node_rc) = queue.pop_front() {
        let mut node = node_rc.borrow_mut();

        // Swap left and right children
        let temp = node.left.take();
        node.left = node.right.take();
        node.right = temp;

        // Enqueue children if they exist
        if let Some(left) = &node.left {
            queue.push_back(left.clone());
        }
        if let Some(right) = &node.right {
            queue.push_back(right.clone());
        }
    }

    root
}
