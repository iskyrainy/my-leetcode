use crate::lc::ListNode;

struct MedianFinder {
    size: usize,
    head: Option<Box<ListNode>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            size: 0,
            head: None,
        }
    }

    fn add_num(&mut self, num: i32) {
        let mut curr = &mut self.head;
        while curr.as_ref().is_some_and(|node| node.val < num) {
            curr = &mut curr.as_mut().unwrap().next;
        }
        let old_next = curr.take();
        *curr = Some(Box::new(ListNode {
            val: num,
            next: old_next,
        }));
        self.size += 1;
    }

    fn find_median(&mut self) -> f64 {
        let mid = self.size / 2;
        let mut curr = self.head.as_ref();
        if self.size % 2 == 1 {
            for _ in 0..mid {
                curr = curr.unwrap().next.as_ref();
            }
            curr.unwrap().val as f64
        } else {
            for _ in 0..mid - 1 {
                curr = curr.unwrap().next.as_ref();
            }
            let v1 = curr.unwrap().val;
            let v2 = curr.unwrap().next.as_ref().unwrap().val;
            (v1 + v2) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_295::MedianFinder;

    #[test]
    fn test_find_median_1() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(1.5_f64, mf.find_median());
        mf.add_num(3);
        assert_eq!(2_f64, mf.find_median());
    }
}
