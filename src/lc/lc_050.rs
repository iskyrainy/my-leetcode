pub fn my_pow(x: f64, n: i32) -> f64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_050::my_pow;

    #[test]
    fn test_my_pow_1() {
        assert_eq!(1024.00000, my_pow(2.0, 10));
    }
}
