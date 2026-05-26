pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    
}

#[cfg(test)]
mod test {
    use crate::lc::lc_048::rotate;

    #[test]
    fn test_rotate_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix);
    }
}
