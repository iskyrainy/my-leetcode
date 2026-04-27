pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn dfs(i: usize, k: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        let d = k - path.len();
        if d == 0 {
            ans.push(path.to_vec());
            return;
        }
        for j in (d..=i).rev() {
            path.push(j as i32);
            dfs(j - 1, k, path, ans);
            path.pop();
        }
    }
    let mut path = vec![];
    let mut ans = vec![];
    dfs(n as usize, k as usize, &mut path, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use crate::lc::lc_077::combine;

    #[test]
    fn test_combine_1() {
        assert_eq!(
            vec![
                vec![4, 3], 
                vec![4, 2], 
                vec![4, 1], 
                vec![3, 2], 
                vec![3, 1], 
                vec![2, 1]
            ],
            combine(4, 2)
        );
    }

    #[test]
    fn test_combine_2() {
        assert_eq!(vec![vec![1]], combine(1, 1));
    }
}
