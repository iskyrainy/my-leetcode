pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut nums = [0; 26];
    for ch in magazine.chars() {
        nums[ch as usize - 97] += 1;
    }
    for ch in ransom_note.chars() {
        nums[ch as usize - 97] -= 1;
        if nums[ch as usize - 97] < 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_383::can_construct;

    #[test]
    fn test_can_construct_1() {
        assert!(!can_construct(String::from("a"), String::from("b")));
        assert!(can_construct(String::from("a"), String::from("a")));
        assert!(!can_construct(String::from("ab"), String::from("a")));
    }
}
