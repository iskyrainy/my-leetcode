use std::cmp::Reverse;

use crate::Heap;

pub struct SmallestInfiniteSet {
    lazy: i32,
    heap: Heap<Reverse<i32>>,
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        let heap = Heap::new(1000);
        SmallestInfiniteSet { lazy: 1, heap }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if self.heap.is_empty() {
            let res = self.lazy;
            self.lazy += 1;
            return res;
        } else {
            if let Some(Reverse(s)) = self.heap.pop() {
                return s;
            }
        }
        0
    }

    pub fn add_back(&mut self, num: i32) {
        if num < self.lazy && !self.heap.data.contains(&Reverse(num)) {
            self.heap.push(Reverse(num));
        }
    }
}

impl Default for SmallestInfiniteSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::lc_2k::lc_2336::SmallestInfiniteSet;

    #[test]
    fn test_smallest_infinite_set_1() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);
        assert_eq!(1, set.pop_smallest());
        assert_eq!(2, set.pop_smallest());
        assert_eq!(3, set.pop_smallest());
        set.add_back(1);
        assert_eq!(1, set.pop_smallest());
        assert_eq!(4, set.pop_smallest());
        assert_eq!(5, set.pop_smallest());
    }
}
