pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        digits[i] = (digits[i] + 1) % 10;
        if digits[i] != 0 {
            return digits;
        }
    }
    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_066::plus_one;

    #[test]
    fn test_plus_one_1() {
        assert_eq!(vec![1, 2, 4], plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], plus_one(vec![4, 3, 2, 1]));
        assert_eq!(vec![1, 0], plus_one(vec![9]));
    }
}
