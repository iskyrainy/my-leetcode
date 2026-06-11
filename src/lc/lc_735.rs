pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut res = vec![];
    for a in asteroids {
        if a > 0 {
            stack.push(a);
        } else {
            loop {
                if let Some(top) = stack.pop() {
                    if top + a > 0 {
                        stack.push(top);
                        break;
                    } else if top + a == 0 {
                        break;
                    }
                } else {
                    res.push(a);
                    break;
                }
            }
        }
    }
    if !stack.is_empty() {
        res.append(&mut stack);
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_735::asteroid_collision;

    #[test]
    fn test_asteroid_collision_1() {
        assert_eq!(vec![-6, 2, 4], asteroid_collision(vec![3, 5, -6, 2, -1, 4]));
    }
}
