/// 给你一个字符串 s 和一个字符串列表 wordDict 作为字典。如果可以利用字典中出现的一个或多个单词拼接出 s 则返回 true。
/// 注意：不要求字典中出现的单词全部都使用，并且字典中的单词可以重复使用。
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    if n == 0 {
        return true;
    }
    let dict_set: std::collections::HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 0..n {
        if !dp[i] {
            continue;
        }
        for j in i + 1..=n {
            if dict_set.contains(&s[i..j]) {
                dp[j] = true;
            }
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
        assert!(word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
        assert!(!word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string()
            ]
        ));
        assert!(word_break("a".to_string(), vec!["a".to_string()]));
        assert!(!word_break("a".to_string(), vec!["b".to_string()]));
        assert!(word_break("".to_string(), vec![]));
        assert!(word_break(
            "a".to_string(),
            vec!["a".to_string(), "ab".to_string()]
        ));
        assert!(!word_break(
            "abcd".to_string(),
            vec!["abc".to_string(), "de".to_string()]
        ));
        assert!(word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string(), "le".to_string()]
        ));
    }
}
