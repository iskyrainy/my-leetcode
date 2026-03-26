use crate::lc::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut node = &mut dummy;
    for _ in 1..=k {
        if node.next.is_some() {
            node = node.next.as_mut().unwrap();
        } else {
            return dummy.next;
        }
    }
    let last_nodes = reverse_k_group(node.next.take(), k);
    let mut node = &mut dummy;
    let mut cur = node.next.take();
    let mut prev = None;
    let mut next_node;
    for _ in 1..=k {
        if let Some(mut tmp) = cur {
            next_node = tmp.next.take();
            tmp.next = prev;
            prev = Some(tmp);
            cur = next_node;
        }
    }
    node.next = prev;
    while node.next.is_some() {
        node = node.next.as_mut().unwrap();
    }
    node.next = last_nodes;
    dummy.next
}

#[cfg(test)]
mod test {
    use crate::lc::{lc_025::reverse_k_group, to_list, to_vec};

    #[test]
    fn test_reverse_k_group_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![2, 1, 4, 3, 5], to_vec(reverse_k_group(head, 2)));
    }

    #[test]
    fn test_reverse_k_group_2() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![3, 2, 1, 4, 5], to_vec(reverse_k_group(head, 3)));
    }
}
