pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    for num in &nums[1..] {
        res ^= num;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_136::single_number;

    #[test]
    fn test_single_number_1() {
        assert_eq!(1, single_number(vec![2, 2, 1]));
    }
    #[test]
    fn test_single_number_2() {
        assert_eq!(4, single_number(vec![4, 1, 2, 1, 2]));
    }
    #[test]
    fn test_single_number_3() {
        assert_eq!(1, single_number(vec![1]));
    }
}
