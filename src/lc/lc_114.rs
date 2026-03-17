use std::{cell::RefCell, rc::Rc};

use crate::lc::TreeNode;

/// only consider the check-node's left not None
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut check = root.clone();
    while let Some(node) = check {
        let mut node_borrow = node.borrow_mut();
        if let Some(left) = node_borrow.left.take() {
            let mut pre = left.clone();
            loop {
                let nrr = pre.borrow().right.clone();
                if let Some(r) = nrr {
                    pre = r;
                } else {
                    break;
                }
            }
            pre.borrow_mut().right = node_borrow.right.take();
            node_borrow.right = Some(left);
        }
        check = node_borrow.right.clone();
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::lc::{lc_114::flatten, TreeNode};

    #[test]
    fn test_flatten_1() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        {
            let root = root.clone().unwrap();
            root.borrow_mut().left = left;
            root.borrow_mut().right = right;
        }
        flatten(&mut root);
        {
            let root = root.clone().unwrap();
            assert_eq!(0, root.borrow().val);
            let node1 = root.borrow().right.clone().unwrap();
            assert_eq!(1, node1.borrow().val);
            let node2 = root
                .borrow()
                .right
                .clone()
                .unwrap()
                .borrow()
                .right
                .clone()
                .unwrap();
            assert_eq!(2, node2.borrow().val);
        }
    }
}
