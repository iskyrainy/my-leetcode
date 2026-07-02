use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::VecDeque;
    if root.is_none() {
        return 0;
    }
    let mut layers = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((root, 0));
    while let Some((Some(node), level)) = queue.pop_front() {
        let borrowed = node.borrow();
        if level == layers.len() {
            layers.push(borrowed.val);
        } else {
            layers[level] += borrowed.val;
        }
        if let Some(left) = borrowed.left.clone() {
            queue.push_back((Some(left), level + 1));
        }
        if let Some(right) = borrowed.right.clone() {
            queue.push_back((Some(right), level + 1));
        }
    }
    1 + layers
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| a.cmp(b).then_with(|| j.cmp(i)))
        .unwrap()
        .0 as i32
}

#[cfg(test)]
mod test {
    use crate::{lc_1k::lc_1161::max_level_sum, to_tree};

    #[test]
    fn test_max_level_sum_1() {
        let root = to_tree(vec![1, 7, 0, 7, -8, -1, -1]);
        assert_eq!(2, max_level_sum(root));
    }
}
