pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let (mut start, mut end) = (0_usize, 0_usize);
    let chs: Vec<char> = s.chars().collect();
    let expand = |s: &Vec<char>, mut left: usize, mut right: usize| -> usize {
        while left > 0 && right < s.len() - 1 && s[left] == s[right] {
            left -= 1;
            right += 1;
        }
        if s[left] == s[right] {
            right - left + 1
        } else {
            right - left - 1
        }
    };
    for i in 0..s.len() - 1 {
        let len = std::cmp::max(expand(&chs, i, i), expand(&chs, i, i + 1));
        if len > end - start {
            (start, end) = (i - (len - 1) / 2, i + len / 2);
        }
    }
    String::from(&s[start..=end])
}

#[cfg(test)]
mod test {
    use crate::lc::lc_005::longest_palindrome;

    #[test]
    fn test_longest_palindrome_1() {
        assert_eq!(
            String::from("aba"),
            longest_palindrome(String::from("babad"))
        );
    }

    #[test]
    fn test_longest_palindrome_2() {
        assert_eq!(String::from("bb"), longest_palindrome(String::from("cbbd")));
    }
}
