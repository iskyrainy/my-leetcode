pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::with_capacity(k as usize);
    let mut v = Vec::with_capacity(k as usize);
    nums1
        .iter()
        .enumerate()
        .take(std::cmp::min(nums1.len(), k as usize))
        .for_each(|(i, _)| heap.push((-nums1[i] - nums2[0], i, 0)));
    for _ in 0..k {
        let (_, i, j) = heap.pop().unwrap();
        v.push(vec![nums1[i], nums2[j]]);
        if j + 1 < nums2.len() {
            heap.push((-nums1[i] - nums2[j + 1], i, j + 1));
        }
    }
    v
}

#[cfg(test)]
mod test {
    use crate::lc::lc_373::k_smallest_pairs;

    #[test]
    fn test_k_smallest_pairs_1() {
        assert_eq!(
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
        );
    }

    #[test]
    fn test_k_smallest_pairs_2() {
        assert_eq!(
            vec![vec![1, 1], vec![1, 1]],
            k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2)
        );
    }
}
