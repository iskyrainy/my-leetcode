pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    (nums[0] * nums[1] * nums[n - 1]).max(nums[n - 3] * nums[n - 2] * nums[n - 1])
}

#[cfg(test)]
mod test {
    use crate::lc::lc_628::maximum_product;

    #[test]
    fn test_maximum_product_1() {
        assert_eq!(24, maximum_product(vec![1, 2, 3, 4]));
    }
}
