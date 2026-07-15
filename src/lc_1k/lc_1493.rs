use crate::lc_1k::lc_1004;

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    lc_1004::longest_ones(nums, 1)
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1493::longest_subarray;

    #[test]
    fn test_longest_subarray_1() {
        assert_eq!(3, longest_subarray(vec![1, 1, 0, 1]));
    }
}
