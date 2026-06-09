pub fn reverse_vowels(s: String) -> String {
    let chs: Vec<char> = s.chars().collect();
    let mut res_ch = chs.clone();
    let (mut f, mut e) = (0_usize, s.len() - 1);
    fn is_vowel(ch: char) -> bool {
        matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
    while f < e {
        while f < e && !is_vowel(chs[f]) {
            f += 1;
        }
        while f < e && !is_vowel(chs[e]) {
            e -= 1;
        }
        if f < e {
            res_ch.swap(f, e);
            f += 1;
            e -= 1;
        }
    }
    String::from_iter(res_ch)
}

#[cfg(test)]
mod test {
    use crate::lc::lc_345::reverse_vowels;

    #[test]
    fn test_reverse_vowels_1() {
        assert_eq!(
            String::from("leotcede"),
            reverse_vowels(String::from("leetcode"))
        );
    }
}
