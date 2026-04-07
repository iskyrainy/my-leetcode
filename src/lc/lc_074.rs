pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut f = false;
    for line in matrix {
        if line[0] > target {
            return false;
        }
        if line[line.len() - 1] < target {
            continue;
        }
        f = line.contains(&target);
        break;
    }
    f
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::lc::lc_074::search_matrix;

    #[test]
    fn test_search_matrix_1() {
        assert!(search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
    }

    #[test]
    fn test_search_matrix_2() {
        assert!(!search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
