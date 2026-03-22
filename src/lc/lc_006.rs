pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 || num_rows >= s.len() {
        return s;
    }
    let t = num_rows * 2 - 2;
    let mut res = String::new();
    for i in 0..num_rows {
        for j in (0..s.len() - i).step_by(t) {
            res.push_str(&s[j + i..j + i + 1]);
            if (i > 0 && i < num_rows - 1) && j + t - i < s.len() {
                res.push_str(&s[j + t - i..j + t - i + 1]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_006::convert;

    #[test]
    fn test_convert_1() {
        assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PAYPALISHIRING"), 3));
        assert_eq!("PINALSIGYAHRPI", convert(String::from("PAYPALISHIRING"), 4));
        assert_eq!("A", convert(String::from("A"), 1));
    }
}
