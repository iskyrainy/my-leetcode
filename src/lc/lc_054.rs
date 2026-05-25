pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_054::spiral_order;

    #[test]
    fn test_spiral_order_1() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
