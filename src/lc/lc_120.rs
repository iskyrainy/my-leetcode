pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_120::minimum_total;

    #[test]
    fn test_minimun_total_1() {
        assert_eq!(
            11,
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }

    #[test]
    fn test_minimun_total_2() {
        assert_eq!(-10, minimum_total(vec![vec![-10]]));
    }
}
