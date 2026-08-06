pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut i = n;
    loop {
        let mut prod = 1;
        let mut x = i;
        while x > 0 {
            prod *= x % 10;
            x /= 10;
        }
        if prod % t == 0 {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3345::smallest_number;

    #[test]
    fn test_smallest_number_1() {
        assert_eq!(10, smallest_number(10, 2));
        assert_eq!(16, smallest_number(15, 3));
    }
}
