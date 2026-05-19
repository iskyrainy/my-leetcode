pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }
    let mut res = 0;
    points.sort_unstable_by_key(|v| v[0]);
    let (mut start, mut end) = (points[0][0], points[0][1]);
    for point in points.iter().skip(1) {
        if point[0] <= end {
            start = start.max(point[0]);
            end = end.min(point[1]);
        } else {
            (start, end) = (point[0], point[1]);
            res += 1;
        }
    }
    res + 1
}

#[cfg(test)]
mod test {
    use crate::lc::lc_452::find_min_arrow_shots;

    #[test]
    fn test_find_min_arrow_shots_1() {
        assert_eq!(
            2,
            find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
    }
}
