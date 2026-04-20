pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(needle.as_str()) {
        Some(i) => i as _,
        None => -1,
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_028::str_str;

    #[test]
    fn test_str_str_1() {
        assert_eq!(0, str_str(String::from("sadbutsad"), String::from("sad")));
    }

    #[test]
    fn test_str_str_2() {
        assert_eq!(-1, str_str(String::from("leetcode"), String::from("leeto")));
    }
}
