pub fn num_tilings(n: i32) -> i32 {
    let mut dp = vec![0; 2 * n as usize + 1];
    dp[2] = 1;
    if n > 1 {
        dp[3] = 1;
    }
    // dp[2n] = (dp[2n-6] + dp[2n-2] + dp[2n-3])
    for i in 0..=2 * n as usize {
        if i > 6 {
            dp[i] += dp[i - 6];
        }
        if i > 3 {
            dp[i] += dp[i - 3];
        }
        if i > 2 {
            dp[i] += dp[i - 2];
        }
    }
    dp[2 * n as usize]
}

#[cfg(test)]
mod test {
    use crate::lc::lc_790::num_tilings;

    #[test]
    fn test_num_tilings_1() {
        assert_eq!(5, num_tilings(3));
        assert_eq!(1, num_tilings(1));
    }
}
