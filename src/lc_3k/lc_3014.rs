pub fn minimum_pushes(word: String) -> i32 {
    let mut n = word.len();
    let mut i = 1_usize;
    let mut res = 0;
    while n >= 8 {
        res += 8 * i;
        n -= 8;
        i += 1;
    }
    res += n * i;
    res as _
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3014::minimum_pushes;

    #[test]
    fn test_minimum_pushes_1() {
        assert_eq!(5, minimum_pushes(String::from("abcde")));
    }
}
