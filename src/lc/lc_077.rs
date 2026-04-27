pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_077::combine;

    #[test]
    fn test_combine_1() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ],
            combine(4, 2)
        );
    }

    #[test]
    fn test_combine_2() {
        assert_eq!(vec![vec![1]], combine(1, 1));
    }
}
