pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut a = str1.clone();
    a.push_str(str2.as_str());
    let mut b = str2.clone();
    b.push_str(str1.as_str());
    if a.ne(&b) {
        return String::new();
    }
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    str1[0..gcd(str1.len(), str2.len())].to_string()
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1071::gcd_of_strings;

    #[test]
    fn test_gcd_of_strings_1() {
        assert_eq!(
            String::from("ABC"),
            gcd_of_strings(String::from("ABCABC"), String::from("ABC"))
        );
    }
}
