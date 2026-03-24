pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let res = (0.5 * (x as f64).ln()).exp() as i32;
    if (res + 1) as f64 * (res + 1) as f64 <= x as f64 {
        res + 1
    } else {
        res
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_069::my_sqrt;

    #[test]
    fn test_my_sqrt_1() {
        assert_eq!(46340, my_sqrt(2147483647));
    }

    #[test]
    fn test_my_sqrt_2() {
        assert_eq!(46340, my_sqrt(2147395600));
    }
}
