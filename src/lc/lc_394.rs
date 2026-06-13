pub fn decode_string(s: String) -> String {
    let chs: Vec<char> = s.chars().collect();
    let (mut num, mut res) = (String::new(), String::new());
    let (mut i, mut combo) = (0, 0);
    while i < chs.len() {
        let chi = chs[i];
        match chi {
            '[' => {
                let mut times = num.parse::<usize>().unwrap();
                num.clear();
                let mut tmp = String::new();
                combo += 1;
                while combo > 0 {
                    i += 1;
                    let ch = chs[i];
                    if ch == '[' {
                        combo += 1;
                    } else if ch == ']' {
                        combo -= 1;
                        if combo == 0 {
                            break;
                        }
                    }
                    tmp.push(ch);
                }
                let tmp = decode_string(tmp);
                while times > 0 {
                    res.push_str(tmp.as_str());
                    times -= 1;
                }
            }
            '0'..='9' => num.push(chi),
            'a'..='z' => res.push(chi),
            _ => {}
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_394::decode_string;

    #[test]
    fn test_decode_string_1() {
        assert_eq!(
            String::from("abccdcdcdxyz"),
            decode_string(String::from("abc3[cd]xyz"))
        );
        assert_eq!(
            String::from("accaccacc"),
            decode_string(String::from("3[a2[c]]"))
        );
        assert_eq!(
            String::from("aaabcbc"),
            decode_string(String::from("3[a]2[bc]"))
        )
    }
}
