pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for (i, line) in triangle.clone().iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (ii, _) in line.iter().enumerate() {
            if ii == 0 {
                triangle[i][ii] += triangle[i - 1][ii];
            } else if ii == line.len() - 1 {
                triangle[i][ii] += triangle[i - 1][ii - 1];
            } else {
                triangle[i][ii] += std::cmp::min(triangle[i - 1][ii], triangle[i - 1][ii - 1]);
            }
        }
    }
    *triangle[triangle.len() - 1].iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_120::minimum_total;

    #[test]
    fn test_minimun_total_1() {
        assert_eq!(
            11,
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }

    #[test]
    fn test_minimun_total_2() {
        assert_eq!(-10, minimum_total(vec![vec![-10]]));
    }
}
