pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;

    let k = k as usize;
    (0..k).for_each(|i| {
        sum += nums[i];
    });

    let mut max = sum;
    for i in k..nums.len() {
        sum -= nums[i - k];
        sum += nums[i];
        if sum > max {
            max = sum;
        }
    }
    max as f64 / k as f64
}

#[cfg(test)]
mod test {
    use crate::lc::lc_643::find_max_average;

    #[test]
    fn test_find_max_average_1() {
        assert_eq!(12.75_f64, find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
    }
}
