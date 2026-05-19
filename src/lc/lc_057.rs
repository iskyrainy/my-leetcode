pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut start, mut end) = (new_interval[0], new_interval[1]);
    let mut res = Vec::new();
    let mut merged = false;

    for interval in intervals {
        if interval[1] < start {
            res.push(interval);
        } else if interval[0] > end {
            if !merged {
                res.push(vec![start, end]);
                merged = true;
            }
            res.push(interval);
        } else {
            start = start.min(interval[0]);
            end = end.max(interval[1]);
        }
    }

    if !merged {
        res.push(vec![start, end]);
    }

    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_057::insert;

    #[test]
    fn test_insert_1() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }
}
