pub fn max_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    ((nums[0] - 1) * (nums[1] - 1)).max((nums[n - 1] - 1) * (nums[n - 2] - 1))
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1464::max_product;

    #[test]
    fn test_max_product_1() {
        assert_eq!(12, max_product(vec![1, 4, 5, 2]));
    }
}
