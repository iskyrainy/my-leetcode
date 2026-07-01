use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root?;
    let (val, left, right) = {
        let mut cur = root.borrow_mut();
        (cur.val, cur.left.take(), cur.right.take())
    };
    if val == key {
        match (left, right) {
            (None, None) => return None,
            (Some(node), None) | (None, Some(node)) => return Some(node),
            (Some(l), Some(r)) => {
                let mut t = r.clone();
                while let Some(next) = t.clone().borrow().left.as_ref() {
                    t = next.clone();
                }
                t.borrow_mut().left = Some(l);
                return Some(r);
            }
        }
    }
    let mut cur = root.borrow_mut();
    if val > key {
        cur.left = delete_node(left, key);
        cur.right = right;
    } else {
        cur.right = delete_node(right, key);
        cur.left = left;
    }
    drop(cur);
    Some(root)
}

#[cfg(test)]
mod test {
    use crate::{flat_tree_level_order, lc::lc_450::delete_node, to_tree};

    #[test]
    fn test_delete_node_1() {
        let root = to_tree(vec![5, 3, 6, 2, 4, -1, 7]);
        assert_eq!(
            vec![5, 4, 6, 2, 7],
            flat_tree_level_order(delete_node(root, 3))
        );
    }
}
