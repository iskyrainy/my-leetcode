pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap();
    let mut res = vec![false; candies.len()];
    candies.iter().enumerate().for_each(|(i, &c)| {
        if c + extra_candies >= *max {
            res[i] = true;
        }
    });
    res
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1431::kids_with_candies;

    #[test]
    fn test_kids_with_candies_1() {
        assert_eq!(
            vec![true, true, true, false, true],
            kids_with_candies(vec![2, 3, 5, 1, 3], 3)
        );
    }
}
