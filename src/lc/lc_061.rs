use crate::lc::ListNode;

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut cur = &mut dummy;
    let mut length = 0;
    while let Some(node) = cur.next.as_mut() {
        length += 1;
        cur = node;
    }
    let k = length - k % length;
    if k == 0 || k == length {
        return dummy.next;
    }
    let mut cur = &mut dummy;
    for _ in 1..k {
        cur = cur.next.as_mut().unwrap();
    }
    let mut p = cur.next.take();
    let mut next_node;
    let mut prev = None;
    for _ in k..=length {
        if let Some(mut tmp) = p {
            next_node = tmp.next.take();
            tmp.next = prev;
            prev = Some(tmp);
            p = next_node;
        }
    }
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::{lc_061::rotate_right, to_list, to_vec};

    #[test]
    fn test_rotate_right_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![4, 5, 1, 2, 3], to_vec(rotate_right(head, 2)));
    }

    #[test]
    fn test_rotate_right_2() {
        let head = to_list(vec![0, 1, 2]);
        assert_eq!(vec![2, 0, 1], to_vec(rotate_right(head, 4)));
    }
}
