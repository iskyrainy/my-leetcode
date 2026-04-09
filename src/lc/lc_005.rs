pub fn longest_palindrome(s: String) -> String {
    // dp[0][n-1] = max(dp[1][n-2], dp[n-2][1])
    // ...
    // 
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_005::longest_palindrome;

    #[test]
    fn test_longest_palindrome_1() {
        assert_eq!(
            String::from("bab"),
            longest_palindrome(String::from("babad"))
        );
    }

    #[test]
    fn test_longest_palindrome_2() {
        assert_eq!(String::from("bb"), longest_palindrome(String::from("cbbd")));
    }
}
