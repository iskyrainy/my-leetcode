use crate::lc::ListNode;

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
    use crate::lc::{lc_086::partition, to_list, to_vec};

    #[test]
    fn test_partition_1() {
        let head = to_list(vec![1, 4, 3, 2, 5]);
        let nhead = partition(head, 3);
        assert_eq!(to_vec(nhead), vec![1, 2, 4, 3, 5]);
    }
}
