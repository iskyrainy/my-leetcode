pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let mut res = 0;
    let mut visited = vec![false; is_connected.len()];
    let mut queue = VecDeque::with_capacity(200);
    queue.push_back(0);
    loop {
        while let Some(city) = queue.pop_front() {
            visited[city] = true;
            is_connected[city]
                .iter()
                .enumerate()
                .for_each(|(i, &status)| {
                    if !visited[i] && status == 1 && i != city {
                        queue.push_back(i);
                    }
                });
        }
        res += 1;
        if let Some((i, _)) = visited.iter().enumerate().find(|&(_, &v)| !v) {
            queue.push_back(i);
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_547::find_circle_num;

    #[test]
    fn test_find_circle_num_1() {
        assert_eq!(
            2,
            find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
        );
    }
}
