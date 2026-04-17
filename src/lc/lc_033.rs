pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0_usize, nums.len() - 1);
    while left <= right {
        let mid = (left + right) / 2;
        dbg!(mid);
        if nums[mid] == target {
            return mid as _;
        } else if nums[0] <= nums[mid] {
            if nums[0] <= target && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= *nums.last().unwrap() {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use crate::lc::lc_033::search;

    #[test]
    fn test_search_1() {
        assert_eq!(4, search(vec![4, 5, 0, 1, 2, 3], 2));
        assert_eq!(4, search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }
}
