pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, nums.iter().sum::<i32>());
    for (i, &n) in nums.iter().enumerate() {
        right -= n;
        if left == right {
            return i as _;
        }
        left += n;
    }
    -1
}

#[cfg(test)]
mod test {
    use crate::lc::lc_724::pivot_index;

    #[test]
    fn test_pivot_index_1() {
        assert_eq!(3, pivot_index(vec![1, 7, 3, 6, 5, 6]));
    }

    #[test]
    fn test_pivot_index_2() {
        assert_eq!(-1, pivot_index(vec![1, 2, 3]));
    }

    #[test]
    fn test_pivot_index_3() {
        assert_eq!(2, pivot_index(vec![-1, -1, -1, -1, -1, 0]));
    }
}
