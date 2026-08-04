pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let n_usize = n as usize;
    let k_usize = k as usize;
    let mut edges = vec![Vec::new(); n_usize];
    let mut in_degree = vec![0; n_usize];

    for inv in &invocations {
        let (u, v) = (inv[0] as usize, inv[1] as usize);
        edges[u].push(v);
        in_degree[v] += 1;
    }

    let mut queue = VecDeque::new();
    queue.push_back(k_usize);
    let mut suspicious = vec![false; n_usize];
    suspicious[k_usize] = true;

    while let Some(u) = queue.pop_front() {
        for &v in &edges[u] {
            in_degree[v] -= 1;

            if !suspicious[v] {
                queue.push_back(v);
                suspicious[v] = true;
            }
        }
    }

    let mut can_remove_all = true;
    let mut remaining = Vec::new();

    for i in 0..n_usize {
        if suspicious[i] && in_degree[i] > 0 {
            can_remove_all = false;
            break;
        } else if !suspicious[i] {
            remaining.push(i as i32);
        }
    }

    if !can_remove_all {
        return (0..n).collect();
    }

    remaining
}

#[cfg(test)]
mod test {
    use crate::lc_3k::lc_3310::remaining_methods;

    #[test]
    fn test_remaining_methods_1() {
        assert_eq!(
            vec![3, 4],
            remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]])
        );
    }
}
