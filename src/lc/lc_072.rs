pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    let ch1: Vec<char> = word1.chars().collect();
    let ch2: Vec<char> = word2.chars().collect();
    for i in 0..=word1.len() {
        dp[i][0] = i;
    }
    for i in 0..=word2.len() {
        dp[0][i] = i;
    }
    for i in 1..=word1.len() {
        for j in 1..=word2.len() {
            if ch1[i - 1] == ch2[j - 1] {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1]);
            } else {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }
    }
    dp[word1.len()][word2.len()] as _
}

#[cfg(test)]
mod test {
    use crate::lc::lc_072::min_distance;

    #[test]
    fn test_min_distance_1() {
        assert_eq!(3, min_distance(String::from("horse"), String::from("ros")));
    }

    #[test]
    fn test_min_distance_2() {
        assert_eq!(
            5,
            min_distance(String::from("intention"), String::from("execution"))
        );
    }
}
