pub fn reverse_bits(n: i32) -> i32 {
    n.reverse_bits()
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_190::reverse_bits;

    #[test]
    fn test_reverse_bits_1() {
        assert_eq!(964176192, reverse_bits(43261596));
    }

    #[test]
    fn test_reverse_bits_2() {
        assert_eq!(1073741822, reverse_bits(2147483644));
    }
}

