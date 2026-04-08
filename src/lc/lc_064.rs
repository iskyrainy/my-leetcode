pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut dg = grid.clone();
    let (m, n) = (grid.len(), grid[0].len());
    for i in 1..n {
        dg[0][i] += dg[0][i - 1];
    }
    for i in 1..m {
        dg[i][0] += dg[i - 1][0];
        for ii in 1..n {
            dg[i][ii] += std::cmp::min(dg[i][ii - 1], dg[i - 1][ii]);
        }
    }
    dg[m - 1][n - 1]
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
