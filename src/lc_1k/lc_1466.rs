pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1466::min_reorder;

    #[test]
    fn test_min_reorder_1() {
        assert_eq!(
            3,
            min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            )
        );
    }
}
