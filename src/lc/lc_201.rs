pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut i = 0;
    while i < 32 {
        if left >> i == right >> i {
            break;
        }
        i += 1;
    }
    left >> i << i
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_201::range_bitwise_and;

    #[test]
    fn test_range_bitwise_and_1() {
        assert_eq!(4, range_bitwise_and(5, 7));
        assert_eq!(0, range_bitwise_and(3, 5));
        assert_eq!(0, range_bitwise_and(1, 2147483647));
    }
}
