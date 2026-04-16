use std::cmp::Reverse;

use crate::lc::Heap;

#[derive(Debug)]
struct MedianFinder {
    min: Heap<i32>,
    max: Heap<Reverse<i32>>,
    size: usize,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min: Heap::new(25001),
            max: Heap::new(25001),
            size: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.min.is_empty() || num <= *self.min.peek().unwrap() {
            self.min.push(num);
            if self.max.len() + 1 < self.min.len() {
                self.max.push(Reverse(self.min.pop().unwrap()));
            }
        } else {
            self.max.push(Reverse(num));
            if self.max.len() > self.min.len() {
                self.min.push(self.max.pop().unwrap().0);
            }
        }
    }

    fn find_median(&mut self) -> f64 {
        let min_top = *self.min.peek().unwrap();
        if self.min.len() > self.max.len() {
            return min_top as _;
        }
        (min_top + self.max.peek().unwrap().0) as f64 / 2.0
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
