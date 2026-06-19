use crate::lc::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut pre, mut cur) = (None, head);
    while let Some(mut node) = cur {
        let next = node.next;
        node.next = pre;
        pre = Some(node);
        cur = next;
    }
    pre
}

#[cfg(test)]
mod test {
    use crate::lc::{lc_206::reverse_list, to_list, to_vec};

    #[test]
    fn test_reverse_list_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![5, 4, 3, 2, 1], to_vec(reverse_list(head)));
    }
}
