use std::{collections::HashMap, sync::LazyLock};

static MAP: LazyLock<HashMap<char, Vec<char>>> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(9);
    m.insert('1', Vec::new());
    m.insert('2', vec!['a', 'b', 'c']);
    m.insert('3', vec!['d', 'e', 'f']);
    m.insert('4', vec!['g', 'h', 'i']);
    m.insert('5', vec!['j', 'k', 'l']);
    m.insert('6', vec!['m', 'n', 'o']);
    m.insert('7', vec!['p', 'q', 'r', 's']);
    m.insert('8', vec!['t', 'u', 'v']);
    m.insert('9', vec!['w', 'x', 'y', 'z']);
    m
});

pub fn letter_combinations(digits: String) -> Vec<String> {
    let chs: Vec<char> = digits.chars().collect();
    let mut res = vec![];
    for i in 0..chs.len() {
        let first = MAP.get(&chs[i]).unwrap();
        chs.iter().skip(i + 1).for_each(|n| {
            let second = MAP.get(n).unwrap();
            first.iter().for_each(|f| {
                second
                    .iter()
                    .for_each(|s| res.push(format!("{}{}", *f, *s)))
            });
        });
        if chs.len() == 1 {
            first.iter().for_each(|c| res.push(String::from(*c)));
        }
    }
    res
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
