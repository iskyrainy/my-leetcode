pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;
    let m = 10_usize.pow(9) + 7;
    let mut dp = vec![vec![0; 4]; n + 1];
    dp[0][3] = 1;
    for i in 1..=n {
        dp[i][0] = dp[i - 1][3];
        dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % m;
        dp[i][2] = (dp[i - 1][0] + dp[i - 1][1]) % m;
        dp[i][3] = (dp[i][2] + dp[i - 1][2] + dp[i - 1][3]) % m;
    }
    dp[n][3] as _
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
