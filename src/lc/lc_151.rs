pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_151::reverse_words;

    #[test]
    fn test_reverse_words_1() {
        assert_eq!("world hello", reverse_words(String::from("hello   world")));
        assert_eq!(
            "blue is sky the",
            reverse_words(String::from("the sky is blue"))
        );
        assert_eq!(
            "world hello",
            reverse_words(String::from("  hello world  "))
        );
    }
}
