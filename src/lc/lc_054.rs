pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut left, mut right, mut top, mut bottom) = (0_usize, n - 1, 0_usize, m - 1);
    while left <= right && top <= bottom {
        (left..=right).for_each(|i| {
            res.push(matrix[top][i]);
        });
        ((top + 1)..=bottom).for_each(|i| {
            res.push(matrix[i][right]);
        });
        if left < right && top < bottom {
            for i in ((left + 1)..right).rev() {
                res.push(matrix[bottom][i]);
            }
            for i in ((top + 1)..=bottom).rev() {
                res.push(matrix[i][left]);
            }
        }
        left += 1;
        right = right.saturating_sub(1);
        top += 1;
        bottom = bottom.saturating_sub(1);
    }
    res
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

    #[test]
    fn test_spiral_order_2() {
        assert_eq!(vec![1], spiral_order(vec![vec![1]]));
    }
}
