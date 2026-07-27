pub fn max_product(n: i32) -> i32 {
    let mut s = n.to_string().chars().collect::<Vec<char>>();
    s.sort_unstable_by_key(|&a| std::cmp::Reverse(a));
    (s[0] as i32 - 48) * (s[1] as i32 - 48)
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3536::max_product;

    #[test]
    fn test_max_product_1() {
        assert_eq!(3, max_product(31));
    }
}
