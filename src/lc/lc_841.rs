pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;
    let mut visited = vec![false; rooms.len()];
    let mut queue = VecDeque::with_capacity(3000);
    queue.push_back(0);
    while let Some(key) = queue.pop_front() {
        let key = key as usize;
        let room = &rooms[key];
        visited[key] = true;
        room.iter().for_each(|&k| {
            if !visited[k as usize] {
                queue.push_back(k);
            }
        });
    }
    !visited.iter().any(|f| !*f)
}

#[cfg(test)]
mod test {
    use crate::lc::lc_841::can_visit_all_rooms;

    #[test]
    fn test_can_visit_all_rooms_1() {
        assert!(can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
    }
}
