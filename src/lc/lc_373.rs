pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_373::k_smallest_pairs;

    #[test]
    fn test_k_smallest_pairs_1() {
        assert_eq!(
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
        );
    }
}
