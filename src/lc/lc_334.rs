use crate::lc::lc_300::length_of_lis;

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    length_of_lis(nums) > 2
}

#[cfg(test)]
mod test {
    use crate::lc::lc_334::increasing_triplet;

    #[test]
    fn test_increasing_triplet_1() {
        assert!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }
}
