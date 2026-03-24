pub fn trailing_zeroes(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        n /= 5;
        count += n;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_172::trailing_zeroes;

    #[test]
    fn test_trailing_zeros_1() {
        assert_eq!(24, trailing_zeroes(100));
        assert_eq!(49, trailing_zeroes(200));
        assert_eq!(0, trailing_zeroes(0));
        assert_eq!(2, trailing_zeroes(10));
    }

    #[test]
    fn test_trailing_zeros_2() {
        assert_eq!(7, trailing_zeroes(30));
    }
}
