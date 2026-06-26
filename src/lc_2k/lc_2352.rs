pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let n = grid.len();
    let mut map: HashMap<&Vec<i32>, i32> = HashMap::with_capacity(n);
    for line in &grid {
        if map.contains_key(line) {
            map.insert(line, 1 + *map.get(line).unwrap());
        } else {
            map.insert(line, 1);
        }
    }
    let mut res = 0;
    (0..n).for_each(|j| {
        let mut col = vec![];
        (0..n).for_each(|i| {
            col.push(grid[i][j]);
        });
        if let Some(t) = map.get(&col) {
            res += *t;
        }
    });
    res
}

#[cfg(test)]
mod test {
    use crate::lc_2k::lc_2352::equal_pairs;

    #[test]
    fn test_equal_pairs_1() {
        assert_eq!(
            1,
            equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]])
        );
    }
}
