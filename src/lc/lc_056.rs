pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    intervals.sort_by_key(|x| x[0]);
    let mut curr = intervals[0].clone();
    for interval in intervals.iter().skip(1) {
        if interval[0] <= curr[1] {
            curr[1] = curr[1].max(interval[1]);
        } else {
            res.push(curr);
            curr = interval.clone();
        }
    }
    res.push(curr);
    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_056::merge;

    #[test]
    fn test_merge_1() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
