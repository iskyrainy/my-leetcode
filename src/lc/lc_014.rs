pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let s0 = &strs[0];
    for (j, c) in s0.bytes().enumerate() {
        for s in &strs {
            if j == s.len() || s.as_bytes()[j] != c {
                return s0[..j].to_string();
            }
        }
    }
    s0.clone()
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_014::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix_1() {
        let strs = vec![
            String::from("zhang"),
            String::from("zhangy"),
            String::from("zhangyt"),
        ];
        assert_eq!("zhang", longest_common_prefix(strs).as_str());
    }
}
