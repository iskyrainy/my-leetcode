pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let n = nums.len();
    let mut i = 0;
    let mut ans = vec![];
    while i < n {
        let low = i;
        i += 1;
        while i < n && nums[i] == nums[i - 1] + 1 {
            i += 1;
        }
        let high = i - 1;
        let mut s = nums[low].to_string();
        if low < high {
            s.push_str(format!("->{}", nums[high]).as_str());
        }
        ans.push(s);
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::lc::lc_228::summary_ranges;

    #[test]
    fn test_summary_ranges_1() {
        assert_eq!(
            vec![
                String::from("0->2"),
                String::from("4->5"),
                String::from("7")
            ],
            summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
    }
}
