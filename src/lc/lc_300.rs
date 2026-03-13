/// 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
/// 子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。
/// 例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。
/// nums: 0, 3, 1, 6, 2, 2, 7
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut tails = Vec::new();

    for &num in &nums {
        // Find the first element >= num (lower_bound)
        let mut left = 0;
        let mut right = tails.len();
        while left < right {
            let mid = (left + right) / 2;
            if tails[mid] < num {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // If not found, append num (creates a new longer subsequence)
        if left == tails.len() {
            tails.push(num);
        } else {
            // Replace with num (extends the subsequence and makes it smaller)
            tails[left] = num;
        }
    }

    tails.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_300::length_of_lis;

    #[test]
    fn test_length_of_lis_1() {
        assert_eq!(4, length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn test_length_of_lis_2() {
        assert_eq!(4, length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    }

    #[test]
    fn test_length_of_lis_3() {
        assert_eq!(1, length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
    }
}
