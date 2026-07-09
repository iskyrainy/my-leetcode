pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    use std::collections::BinaryHeap;
    let (k, candidates) = (k as usize, candidates as usize);
    let n = costs.len();
    let mut left = BinaryHeap::with_capacity(candidates);
    let mut right = BinaryHeap::with_capacity(candidates);
    let (mut left_idx, mut right_idx) = (0, n);
    for _ in 0..candidates {
        left.push(-costs[left_idx]);
        left_idx += 1;
    }
    for _ in 0..candidates {
        if right_idx > left_idx {
            right_idx -= 1;
            right.push(-costs[right_idx]);
        }
    }
    let mut res = 0_i64;
    for _ in 0..k {
        let lp = left.peek().unwrap_or(&i32::MIN);
        let rp = right.peek().unwrap_or(&i32::MIN);
        if *lp >= *rp {
            res -= left.pop().unwrap() as i64;
            if left_idx < right_idx {
                left.push(-costs[left_idx]);
                left_idx += 1;
            }
        } else {
            res -= right.pop().unwrap() as i64;
            if left_idx < right_idx {
                right_idx -= 1;
                right.push(-costs[right_idx]);
            }
        }
    }
    res as _
}

#[cfg(test)]
mod test {
    use crate::lc_2k::lc_2462::total_cost;

    #[test]
    fn test_total_cost_1() {
        assert_eq!(11, total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4));
        assert_eq!(
            423,
            total_cost(
                vec![
                    31, 25, 72, 79, 74, 65, 84, 91, 18, 59, 27, 9, 81, 33, 17, 58
                ],
                11,
                2
            )
        );
    }
}
