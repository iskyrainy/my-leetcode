pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let ch1: Vec<char> = s1.chars().collect();
    let ch2: Vec<char> = s2.chars().collect();
    let ch3: Vec<char> = s3.chars().collect();
    let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    dp[0][0] = true;

    for i in 0..=s1.len() {
        for j in 0..=s2.len() {
            if i > 0 {
                dp[i][j] |= dp[i - 1][j] && ch1[i - 1] == ch3[i + j - 1];
            }
            if j > 0 {
                dp[i][j] |= dp[i][j - 1] && ch2[j - 1] == ch3[i + j - 1];
            }
        }
    }
    dp[s1.len()][s2.len()]
}

#[cfg(test)]
mod test {
    use crate::lc::lc_097::is_interleave;

    #[test]
    fn test_is_interleave_1() {
        assert!(is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbcbcac")
        ));
    }

    #[test]
    fn test_is_interleave_2() {
        assert!(!is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbcbccc")
        ));
    }

    #[test]
    fn test_is_interleave_3() {
        assert!(is_interleave(
            String::from(""),
            String::from(""),
            String::from("")
        ));
    }
}
