pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut rows, mut cols) = (vec![false; m], vec![false; n]);
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                rows[i] = true;
                cols[j] = true;
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if rows[i] || cols[j] {
                matrix[i][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_073::set_zeroes;

    #[test]
    fn test_set_zeroes_1() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            matrix
        );
    }
}
