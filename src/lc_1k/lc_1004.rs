pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut max = 0;
    let mut k = k as usize;
    let (mut f, mut t) = (0_usize, 0_usize);
    while f < nums.len() {
        let num = nums[f];
        match num {
            0 if k > 0 => {
                max += 1;
                k -= 1;
                f += 1;
            }
            0 if k == 0 => {
                if max > res {
                    res = max;
                }
                if nums[t] == 0 {
                    k += 1;
                }
                max -= 1;
                t += 1;
            }
            _ => {
                max += 1;
                f += 1;
            }
        }
    }
    res.max(max)
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1004::longest_ones;

    #[test]
    fn test_longest_ones_1() {
        assert_eq!(6, longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2));
    }
}
