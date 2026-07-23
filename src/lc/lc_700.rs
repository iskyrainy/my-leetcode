use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root.clone();
    let mut res = None;
    loop {
        if cur.is_none() {
            break;
        }
        let cur_val = cur.clone().unwrap().borrow().val;
        if cur_val == val {
            res = cur.take();
            break;
        } else if cur_val > val {
            cur = cur.clone().unwrap().borrow().left.clone();
        } else {
            cur = cur.clone().unwrap().borrow().right.clone();
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::{flat_tree_level_order, lc::lc_700::search_bst, to_tree};

    #[test]
    fn test_search_bst_1() {
        let root = to_tree(vec![4, 2, 7, 1, 3]);
        assert_eq!(vec![2, 1, 3], flat_tree_level_order(search_bst(root, 2)));
    }
}
