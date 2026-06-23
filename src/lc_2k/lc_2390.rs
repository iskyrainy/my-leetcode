pub fn remove_stars(s: String) -> String {
    let mut stack = Vec::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '*' {
            stack.pop();
            continue;
        } else {
            stack.push(ch);
        }
    }
    String::from_iter(stack)
}

#[cfg(test)]
mod test {
    use crate::lc_2k::lc_2390::remove_stars;

    #[test]
    fn test_remove_stars_1() {
        assert_eq!(
            String::from("lecoe"),
            remove_stars(String::from("leet**cod*e"))
        );
    }
}
