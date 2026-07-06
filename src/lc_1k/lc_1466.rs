pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for e in connections.iter() {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push((b as i32, 1));
        g[b].push((a as i32, 0));
    }
    fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
        let mut ans = 0;
        for &(b, c) in g[a].iter() {
            if b != fa {
                ans += c + dfs(b as usize, a as i32, g);
            }
        }
        ans
    }
    dfs(0, -1, &g)
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1466::min_reorder;

    #[test]
    fn test_min_reorder_1() {
        assert_eq!(
            3,
            min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            )
        );
    }
}
