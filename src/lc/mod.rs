use std::{cell::RefCell, rc::Rc};

pub mod lc_005;
pub mod lc_006;
pub mod lc_014;
pub mod lc_019;
pub mod lc_025;
pub mod lc_035;
pub mod lc_050;
pub mod lc_061;
pub mod lc_063;
pub mod lc_064;
pub mod lc_066;
pub mod lc_069;
pub mod lc_074;
pub mod lc_086;
pub mod lc_092;
pub mod lc_114;
pub mod lc_120;
pub mod lc_136;
pub mod lc_137;
pub mod lc_139;
pub mod lc_146;
pub mod lc_151;
pub mod lc_162;
pub mod lc_172;
pub mod lc_190;
pub mod lc_191;
pub mod lc_198;
pub mod lc_201;
pub mod lc_205;
pub mod lc_215;
pub mod lc_236;
pub mod lc_283;
pub mod lc_300;
pub mod lc_322;
pub mod lc_383;
pub mod lc_724;
pub mod lc_912;

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// vec to linked list
pub fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vector.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// linked list to vec
pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    while let Some(node) = head {
        res.push(node.val);
        head = node.next;
    }
    res
}
