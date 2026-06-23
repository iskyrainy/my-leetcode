use std::{cell::RefCell, rc::Rc};

pub mod lc;
pub mod lc_1k;
pub mod lc_2k;

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

#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new(cap: usize) -> Heap<T> {
        Heap {
            data: Vec::with_capacity(cap),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, item: T) {
        let old_len = self.data.len();
        self.data.push(item);
        self.sift_up(old_len);
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let pop = self.data.pop();
        self.sift_down(0);
        pop
    }

    fn sift_up(&mut self, mut pos: usize) {
        if pos == 0 {
            return;
        }
        let mut parent = (pos - 1) / 2;
        while pos > 0 {
            if self.data[parent] >= self.data[pos] {
                break;
            }
            self.data.swap(parent, pos);
            if parent == 0 {
                break;
            }
            pos = parent;
            parent = (pos - 1) / 2;
        }
    }

    fn sift_down(&mut self, mut pos: usize) {
        let len = self.data.len();
        let mut child = pos * 2 + 2;
        while child < len {
            if self.data[child - 1] < self.data[child] {
                if self.data[child] <= self.data[pos] {
                    break;
                }
                self.data.swap(pos, child);
                pos = child;
            } else {
                if self.data[child - 1] <= self.data[pos] {
                    break;
                }
                self.data.swap(pos, child - 1);
                pos = child - 1;
            }
            child = pos * 2 + 2;
        }
        if child - 1 < len && self.data[child - 1] > self.data[pos] {
            self.data.swap(pos, child - 1);
        }
    }
}

impl<T: Ord> Default for Heap<T> {
    fn default() -> Self {
        Heap::new(0)
    }
}
