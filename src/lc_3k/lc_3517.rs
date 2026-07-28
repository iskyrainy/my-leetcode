pub fn smallest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let partition = bytes.len() / 2;
    let mut bucket = [0; 26];

    for i in 0..partition {
        bucket[(bytes[i] - b'a') as usize] += 1;
    }

    let mut left = String::new();
    (0..26).for_each(|i| {
        if bucket[i] > 0 {
            left.push_str(&((i as u8 + b'a') as char).to_string().repeat(bucket[i]));
        }
    });

    let mid = if !bytes.len().is_multiple_of(2) {
        (bytes[partition] as char).to_string()
    } else {
        String::new()
    };

    let right: String = left.chars().rev().collect();

    left + &mid + &right
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3517::smallest_palindrome;

    #[test]
    fn test_smallest_palindrome_1() {
        assert_eq!(
            String::from("abccba"),
            smallest_palindrome(String::from("acbbca"))
        );
    }
}
