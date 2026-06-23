use crate::ListNode;

pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut cur = &mut dummy;
    let mut n = 0;
    while let Some(node) = &mut cur.next {
        n += 1;
        cur = node;
    }
    let t = n / 2;
    let mut i = 0;
    cur = &mut dummy;
    while i < t {
        cur = cur.next.as_mut().unwrap();
        i += 1;
    }
    cur.next = cur.next.as_mut().unwrap().next.take();
    dummy.next
}

#[cfg(test)]
mod test {
    use crate::{lc_2k::lc_2095::delete_middle, to_list, to_vec};

    #[test]
    fn test_delete_middle_1() {
        let head = to_list(vec![1, 3, 4, 7, 1, 2, 6]);
        assert_eq!(vec![1, 3, 4, 1, 2, 6], to_vec(delete_middle(head)));
    }
}
