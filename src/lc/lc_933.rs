use std::collections::VecDeque;

pub struct RecentCounter {
    queue: VecDeque<i32>,
}
impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            queue: VecDeque::with_capacity(10000),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        let start = if t > 3000 { t - 3000 } else { 0 };
        self.queue.push_back(t);
        while let Some(peek) = self.queue.front() {
            if *peek < start {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as _
    }
}

impl Default for RecentCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_933::RecentCounter;

    #[test]
    fn test_rc_1() {
        let mut rc = RecentCounter::new();
        assert_eq!(1, rc.ping(1));
        assert_eq!(2, rc.ping(100));
        assert_eq!(3, rc.ping(3001));
        assert_eq!(3, rc.ping(3002));
    }
}
