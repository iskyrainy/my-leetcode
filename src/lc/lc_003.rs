pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ans = 0;
    let mut left = 0;
    let mut cnt = [0; 128];
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        cnt[c] += 1;
        while cnt[c] > 1 {
            cnt[s[left] as usize] -= 1;
            left += 1;
        }
        ans = ans.max(right - left + 1);
    }
    ans as _
}
