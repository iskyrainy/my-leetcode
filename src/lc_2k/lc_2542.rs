pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let n = nums1.len();
    let k = k as usize;
    let mut indexes = (0..n).collect::<Vec<_>>();
    indexes.sort_unstable_by(|&i, &j| nums2[j].cmp(&nums2[i]));
    let mut heap = BinaryHeap::with_capacity(k);
    let mut sum = 0_i64;
    for i in 0..k {
        sum += nums1[indexes[i]] as i64;
        heap.push(-nums1[indexes[i]]);
    }
    let mut ans = sum * nums2[indexes[k - 1]] as i64;
    for i in k..n {
        let x = nums1[indexes[i]];
        if x > -heap.peek().unwrap() {
            sum += (x + heap.pop().unwrap()) as i64;
            heap.push(-x);
            ans = ans.max(sum * nums2[indexes[i]] as i64);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::lc_2k::lc_2542::max_score;

    #[test]
    fn test_max_score_1() {
        assert_eq!(12, max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3));
    }
}
