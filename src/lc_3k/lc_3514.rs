pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let mut two = vec![false; 4096];
    let mut res = vec![false; 4096];
    nums.iter()
        .for_each(|&n| nums.iter().for_each(|&o| two[(n ^ o) as usize] = true));
    let two = two
        .iter()
        .enumerate()
        .filter(|&b| *b.1)
        .map(|x| x.0)
        .collect::<Vec<usize>>();
    nums.iter()
        .for_each(|&n| two.iter().for_each(|&t| res[(n as usize) ^ t] = true));
    res.iter().filter(|&&t| t).count() as _
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3514::unique_xor_triplets;

    #[test]
    fn test_unique_xor_triplets_1() {
        assert_eq!(2, unique_xor_triplets(vec![1, 3]));
        assert_eq!(4, unique_xor_triplets(vec![6, 7, 8, 9]));
    }
}
