pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let size = num_courses as usize;
    let mut rect = vec![vec![]; size];
    let mut visited = vec![0; size];
    let mut f = true;
    fn dfs(i: usize, visited: &mut Vec<u8>, rect: &Vec<Vec<usize>>) -> bool {
        let mut f = true;
        visited[i] = 1;
        for v in &rect[i] {
            if visited[*v] == 0 {
                f &= dfs(*v, visited, rect);
            } else if visited[*v] == 1 {
                return false;
            }
        }
        visited[i] = 2;
        f
    }
    for pre in prerequisites {
        let from = pre[1] as usize;
        let to = pre[0] as usize;
        rect[from].push(to);
    }
    for i in 0..visited.len() {
        if visited[i] == 0 {
            f &= dfs(i, &mut visited, &rect);
        }
    }
    f
}

#[cfg(test)]
mod test {
    use crate::lc::lc_207::can_finish;

    #[test]
    fn test_can_finish_1() {
        assert_eq!(true, can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn test_can_finish_2() {
        assert_eq!(false, can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
