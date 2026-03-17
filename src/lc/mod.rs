use std::{cell::RefCell, rc::Rc};

pub mod lc_114;
pub mod lc_136;
pub mod lc_137;
pub mod lc_139;
pub mod lc_146;
pub mod lc_190;
pub mod lc_191;
pub mod lc_198;
pub mod lc_201;
pub mod lc_236;
pub mod lc_300;
pub mod lc_322;

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
