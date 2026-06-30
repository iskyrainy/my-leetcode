pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    ((a & b & !c).count_ones() + ((a | b) ^ c).count_ones()) as _
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1318::min_flips;

    #[test]
    fn test_min_flips_1() {
        assert_eq!(3, min_flips(2, 6, 5));
        assert_eq!(4, min_flips(5, 2, 8));
        assert_eq!(1, min_flips(4, 2, 7));
    }
}
