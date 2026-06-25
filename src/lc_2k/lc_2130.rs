use crate::ListNode;

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut v = vec![0; 100000];
    let mut i = 0;
    let mut max = 0;
    let dummy = Box::new(ListNode { val: 0, next: head });
    let (mut fast, mut slow) = (&dummy, &dummy);
    while let Some(cur_fast) = fast.next.as_ref() {
        fast = cur_fast.next.as_ref().unwrap();
        slow = slow.next.as_ref().unwrap();
        v[i] = slow.val;
        i += 1;
    }
    while let Some(cur_slow) = slow.next.as_ref() {
        i -= 1;
        max = max.max(cur_slow.val + v[i]);
        slow = cur_slow;
    }
    max
}

#[cfg(test)]
mod test {
    use crate::{lc_2k::lc_2130::pair_sum, to_list};

    #[test]
    fn test_pair_sum_1() {
        let head = to_list(vec![5, 4, 2, 1]);
        assert_eq!(6, pair_sum(head));
        let head = to_list(vec![1, 100000]);
        assert_eq!(100001, pair_sum(head));
    }
}
