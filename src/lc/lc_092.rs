use crate::lc::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut p = &mut dummy;
    for _ in 1..left {
        p = p.next.as_mut().unwrap();
    }
    let mut cur = p.next.take();
    let mut prev = None;
    let mut next_node;

    for _ in 0..(right - left + 1) {
        if let Some(mut tmp) = cur {
            next_node = tmp.next.take();
            tmp.next = prev;
            prev = Some(tmp);
            cur = next_node;
        }
    }
    p.next = prev;
    while p.next.is_some() {
        p = p.next.as_mut().unwrap();
    }
    p.next = cur;
    dummy.next
}

#[cfg(test)]
mod tests {
    use crate::lc::{lc_092::reverse_between, to_list, to_vec};

    #[test]
    fn test_reverse_between_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 2, 4);
        assert_eq!(to_vec(result), vec![1, 4, 3, 2, 5]);

        let head2 = to_list(vec![1, 2, 3]);
        let result2 = reverse_between(head2, 1, 2);
        assert_eq!(to_vec(result2), vec![2, 1, 3]);

        let head3 = to_list(vec![5]);
        let result3 = reverse_between(head3, 1, 1);
        assert_eq!(to_vec(result3), vec![5]);
    }
}
