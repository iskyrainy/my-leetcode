pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }
    let (mut w1, mut w2) = (vec![0; 26], vec![0; 26]);
    for (a, b) in word1.chars().zip(word2.chars()) {
        w1[(a as u8 - b'a') as usize] += 1;
        w2[(b as u8 - b'a') as usize] += 1;
    }
    for i in 0..w1.len() {
        if w1[i] > 0 && w2[i] == 0 || w1[i] == 0 && w2[i] > 0 {
            return false;
        }
    }
    w1.sort_unstable();
    w2.sort_unstable();
    for i in 0..w1.len() {
        if w1[i] != w2[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1657::close_strings;

    #[test]
    fn test_close_strings_1() {
        assert!(close_strings(String::from("abc"), String::from("bca")));
    }
}
