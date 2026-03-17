use std::{cell::RefCell, rc::Rc};

use crate::lc::TreeNode;

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut check = root.clone();
    while let Some(node) = check.clone() {
        let right = node.borrow().right.clone();
        if right.is_none() {
            check = node.borrow().left.clone();
            continue;
        }
        let left = node.borrow().left.clone();
        if left.is_none() {
            node.borrow_mut().left = right;
            check = node.borrow().left.clone();
            continue;
        }
        let mut no_right = left.clone();
        loop {
            if no_right.clone().unwrap().borrow().right.is_none() {
                break;
            }
            no_right = node.borrow().right.clone();
        }
        no_right.unwrap().borrow_mut().right = right;
        check = node.borrow().left.clone();
    }
    let root = root.clone().unwrap();
    {
        let mut root = root.borrow_mut();
        root.right = root.left.clone();
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
