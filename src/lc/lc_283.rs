pub fn move_zeroes(nums: &mut Vec<i32>) {
    let (mut left, mut right) = (0_usize, 0_usize);
    while right < nums.len() {
        let num = nums[right];
        if num != 0 {
            nums[right] = nums[left];
            nums[left] = num;
            left += 1;
        }
        right += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_283::move_zeroes;

    #[test]
    fn test_move_zeroes_1() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }

    #[test]
    fn test_move_zeroes_2() {
        let mut nums: Vec<i32> = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(vec![0], nums);
    }
}
