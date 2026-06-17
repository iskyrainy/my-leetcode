pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    let mut res = 0;
    intervals.sort_unstable_by_key(|v| v[0]);
    let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
    for point in intervals.iter().skip(1) {
        if point[0] < end {
            start = start.max(point[0]);
            end = end.min(point[1]);
        } else {
            (start, end) = (point[0], point[1]);
            res += 1;
        }
    }
    (intervals.len() - res - 1) as _
}

#[cfg(test)]
mod test {
    use crate::lc::lc_435::erase_overlap_intervals;

    #[test]
    fn test_erase_overlap_intervals_1() {
        assert_eq!(
            1,
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
        );
    }
}
