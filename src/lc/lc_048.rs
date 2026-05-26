pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut left, mut right, mut top, mut bottom) = (0_usize, n - 1, 0_usize, m - 1);
    let mut tmp = std::collections::VecDeque::new();
    while left <= right && top <= bottom {
        for i in left..=right {
            tmp.push_back(matrix[top][i]);
        }
        for i in top..=bottom {
            tmp.push_back(matrix[i][right]);
        }
        if left < right && top < bottom {
            for i in (left..=right).rev() {
                tmp.push_back(matrix[bottom][i]);
            }
            for i in (top..=bottom).rev() {
                tmp.push_back(matrix[i][left]);
            }
        }
        for i in top..=bottom {
            matrix[i][right] = tmp.pop_front().unwrap();
        }
        if left < right && top < bottom {
            for i in (left..=right).rev() {
                matrix[bottom][i] = tmp.pop_front().unwrap();
            }
            for i in (top..=bottom).rev() {
                matrix[i][left] = tmp.pop_front().unwrap();
            }
        }
        for i in left..=right {
            matrix[top][i] = tmp.pop_front().unwrap();
        }
        left += 1;
        if right > 0 {
            right -= 1;
        }
        top += 1;
        if bottom > 0 {
            bottom -= 1;
        }
    }
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
