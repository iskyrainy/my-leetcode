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

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let (mut less, mut more) = (vec![], vec![]);
    while let Some(node) = head {
        if node.val < x {
            less.push(node.val);
        } else {
            more.push(node.val);
        }
        head = node.next;
    }
    less.append(&mut more);
    let mut nhead = Box::new(ListNode::new(0));
    for &val in less.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = nhead.next;
        nhead.next = Some(node);
    }
    nhead.next
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_086::{ListNode, partition};

    #[test]
    fn test_partition_1() {
        let node5 = Box::new(ListNode::new(5));
        let mut node4 = Box::new(ListNode::new(4));
        let mut node3 = Box::new(ListNode::new(3));
        let mut node2 = Box::new(ListNode::new(2));
        let mut node1 = Box::new(ListNode::new(1));
        node2.next = Some(node5);
        node3.next = Some(node2);
        node4.next = Some(node3);
        node1.next = Some(node4);
        let head = Some(node1);
        // 1, 4, 3, 2, 5

        let mut head = partition(head, 3);

        let mut ordered: Vec<i32> = vec![];
        while let Some(node) = head {
            ordered.push(node.val);
            head = node.next;
        }

        // 1, 2, 4, 3, 5
        assert_eq!(vec![1, 2, 4, 3, 5], ordered);
    }
}
