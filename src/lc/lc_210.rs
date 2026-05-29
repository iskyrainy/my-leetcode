pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let size = num_courses as usize;
    let mut rect = vec![vec![]; size];
    let mut visited = vec![0; size];
    let mut res = vec![];
    let mut f = true;
    fn dfs(i: usize, visited: &mut Vec<u8>, rect: &Vec<Vec<usize>>, res: &mut Vec<i32>) -> bool {
        let mut f = true;
        visited[i] = 1;
        for v in &rect[i] {
            if visited[*v] == 0 {
                f &= dfs(*v, visited, rect, res);
            } else if visited[*v] == 1 {
                return false;
            }
        }
        visited[i] = 2;
        res.insert(0, i as i32);
        f
    }
    for pre in prerequisites {
        let from = pre[1] as usize;
        let to = pre[0] as usize;
        rect[from].push(to);
    }
    for i in 0..visited.len() {
        if visited[i] == 0 {
            f &= dfs(i, &mut visited, &rect, &mut res);
        }
    }
    if !f {
        vec![]
    } else {
        res
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_210::find_order;

    #[test]
    fn test_find_order_1() {
        assert_eq!(vec![0, 1], find_order(2, vec![vec![1, 0]]));
    }

    #[test]
    fn test_find_order_2() {
        assert_eq!(
            vec![0, 2, 1, 3],
            find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
        );
    }
}
