pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let part = |nums: &mut [i32], left: usize, right: usize| -> usize {
        let i = left + rand::random_range(0..=(right - left));
        let pivot = nums[i];
        nums.swap(left, i);
        let (mut i, mut j) = (left + 1, right);
        loop {
            while i <= j && nums[i] < pivot {
                i += 1;
            }
            while i <= j && nums[j] > pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
        nums.swap(left, j);
        j
    };
    let k = nums.len() - k as usize;
    let (mut left, mut right) = (0_usize, nums.len() - 1);
    loop {
        let i = part(&mut nums, left, right);
        if i == k {
            return nums[i];
        } else if i < k {
            left = i + 1;
        } else {
            right = i - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_215::find_kth_largest;

    #[test]
    fn test_find_kth_largest_1() {
        assert_eq!(5, find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
    }

    #[test]
    fn test_find_kth_largest_2() {
        assert_eq!(4, find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4));
    }
}
