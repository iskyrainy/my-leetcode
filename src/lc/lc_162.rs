pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0_usize, nums.len() - 1);
    while left < right {
        let mid = (right + left) / 2;
        if nums[mid] > nums[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as _
}

#[cfg(test)]
mod test {
    use crate::lc::lc_162::find_peak_element;

    #[test]
    fn test_find_peak_element_1() {
        assert_eq!(2, find_peak_element(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_find_peak_element_2() {
        assert_eq!(5, find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
