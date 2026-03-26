use crate::lc::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut node = &mut dummy;
    let mut length = 0;
    while node.next.is_some() {
        node = node.next.as_mut().unwrap();
        length += 1;
    }
    let n = length - n;
    let mut cur = &mut dummy;
    for _ in 1..=n {
        cur = cur.next.as_mut().unwrap();
    }
    let mut del = cur.next.take();
    let del_next = del.as_mut().unwrap().next.take();
    cur.next = del_next;
    dummy.next
}

#[cfg(test)]
mod test {
    use crate::lc::{lc_019::remove_nth_from_end, to_list, to_vec};

    #[test]
    fn test_remove_nth_from_end_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![1, 2, 3, 5], to_vec(remove_nth_from_end(head, 2)));
    }

    #[test]
    fn test_remove_nth_from_end_2() {
        let head = to_list(vec![1]);
        let res: Vec<i32> = Vec::new();
        assert_eq!(res, to_vec(remove_nth_from_end(head, 1)));
    }
}
