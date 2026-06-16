pub struct StockSpanner {
    stocks: Vec<(i32, i32)>,
}

impl StockSpanner {
    pub fn new() -> Self {
        StockSpanner { stocks: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut i = 1;
        if let Some(&peek) = self.stocks.last() {
            
        }
        todo!()
    }
}

impl Default for StockSpanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_901::StockSpanner;

    #[test]
    fn test_stockspanner() {
        let mut ss = StockSpanner::new();
        assert_eq!(1, ss.next(100));
        assert_eq!(1, ss.next(80));
        assert_eq!(1, ss.next(60));
        assert_eq!(2, ss.next(70));
        assert_eq!(1, ss.next(60));
        assert_eq!(4, ss.next(75));
        assert_eq!(6, ss.next(85));
    }
}
