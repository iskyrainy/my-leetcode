use crate::ListNode;

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut odd, mut even) = (Box::new(ListNode::new(0)), Box::new(ListNode::new(0)));
    let (mut cur_odd, mut cur_even) = (&mut odd, &mut even);
    let mut head = head;
    let mut i = 1;
    while let Some(mut cur) = head {
        head = cur.next.take();
        if i % 2 == 1 {
            cur_odd.next = Some(cur);
            cur_odd = cur_odd.next.as_mut().unwrap();
        } else {
            cur_even.next = Some(cur);
            cur_even = cur_even.next.as_mut().unwrap();
        }
        i += 1;
    }
    cur_odd.next = even.next;
    odd.next
}

#[cfg(test)]
mod test {
    use crate::{lc::lc_328::odd_even_list, to_list, to_vec};

    #[test]
    fn test_odd_even_list_1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![1, 3, 5, 2, 4], to_vec(odd_even_list(head)));
    }
}
