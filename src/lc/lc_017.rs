pub fn letter_combinations(digits: String) -> Vec<String> {
    let n = digits.len();
    if n == 0 {
        return vec![];
    }
    fn dfs(i: usize, ans: &mut Vec<String>, path: &mut [u8], digits: &[u8], mapping: &[&str]) {
        if i == digits.len() {
            ans.push(String::from_utf8(path.to_vec()).unwrap());
            return;
        }
        for c in mapping[(digits[i] - b'0') as usize].bytes() {
            path[i] = c;
            dfs(i + 1, ans, path, digits, mapping);
        }
    }
    let mapping = vec!["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut ans = vec![];
    let mut path = vec![0; n];
    dfs(0, &mut ans, &mut path, &digits.as_bytes(), &mapping);
    ans
}

#[cfg(test)]
mod test {
    use crate::lc::lc_017::letter_combinations;

    #[test]
    fn test_letter_combinations_1() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            letter_combinations(String::from("23"))
        );
    }
}
