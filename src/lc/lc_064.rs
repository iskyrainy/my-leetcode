pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_064::min_path_sum;

    #[test]
    fn test_min_path_sum_1() {
        assert_eq!(
            7,
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }

    #[test]
    fn test_min_path_sum_2() {
        assert_eq!(12, min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }
}
