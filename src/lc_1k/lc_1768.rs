pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut s = String::new();
    let (m, n) = (word1.len(), word2.len());
    let (ch1, ch2) = (
        word1.chars().collect::<Vec<char>>(),
        word2.chars().collect::<Vec<char>>(),
    );
    for i in 0..m.max(n) {
        if m > i {
            s.push(ch1[i]);
        }
        if n > i {
            s.push(ch2[i]);
        }
    }
    s
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1768::merge_alternately;

    #[test]
    fn test_merge_alternately_1() {
        assert_eq!(
            String::from("aqswdef"),
            merge_alternately(String::from("asdf"), String::from("qwe"))
        );
    }
}
