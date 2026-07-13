pub fn max_vowels(s: String, k: i32) -> i32 {
    let mut res = 0;
    let mut cnt = 0;
    let k = k as usize;
    let chs: Vec<char> = s.chars().collect();
    (0..k).for_each(|i| match chs[i] {
        'a' | 'e' | 'i' | 'o' | 'u' => cnt += 1,
        _ => {}
    });
    res = cnt;
    for i in k..chs.len() {
        match chs[i] {
            'a' | 'e' | 'i' | 'o' | 'u' => cnt += 1,
            _ => {}
        }
        match chs[i - k] {
            'a' | 'e' | 'i' | 'o' | 'u' => cnt -= 1,
            _ => {}
        }
        if cnt > res {
            res = cnt;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1456::max_vowels;

    #[test]
    fn test_max_vowels_1() {
        assert_eq!(3, max_vowels(String::from("abciiidef"), 3));
    }
}
