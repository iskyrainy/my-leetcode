use std::{cell::RefCell, rc::Rc};

use crate::lc::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref()?;
    let root = root.unwrap();
    let (p, q) = (p.unwrap(), q.unwrap());
    let (p_val, q_val) = (p.borrow().val, q.borrow().val);
    {
        let root_val = root.borrow().val;
        if root_val == p_val || root_val == q_val {
            return Some(root);
        }
    }
    let l =
        self::lowest_common_ancestor(root.borrow().left.clone(), Some(p.clone()), Some(q.clone()));
    let r = self::lowest_common_ancestor(
        root.borrow().right.clone(),
        Some(p.clone()),
        Some(q.clone()),
    );
    if l.is_some() && r.is_some() {
        return Some(root);
    }
    if l.is_none() {
        return r;
    }
    l
}
