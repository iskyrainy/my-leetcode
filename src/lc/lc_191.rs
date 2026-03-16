pub fn hamming_weight(mut n: i32) -> i32 {
    let mut res = 0;
    while n != 0 {
        n &= n-1;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_191::hamming_weight;

    #[test]
    fn test_hamming_weight_1() {
        assert_eq!(3, hamming_weight(11));
    }
    #[test]
    fn test_hamming_weight_2() {
        assert_eq!(1, hamming_weight(128));
    }
    #[test]
    fn test_hamming_weight_3() {
        assert_eq!(30, hamming_weight(2147483645));
    }
}
