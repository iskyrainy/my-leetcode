pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack = Vec::with_capacity(n);
    let mut res = vec![0; n];
    for (i, t) in temperatures.iter().enumerate().rev() {
        if stack.is_empty() {
            res[i] = 0;
            stack.push(i);
        } else {
            loop {
                if let Some(&pi) = stack.last() {
                    if temperatures[pi] > *t {
                        stack.push(i);
                        res[i] = (pi - i) as _;
                        break;
                    } else {
                        stack.pop();
                        continue;
                    }
                } else {
                    stack.push(i);
                    res[i] = 0;
                    break;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc::lc_739::daily_temperatures;

    #[test]
    fn test_daily_temperatures_1() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }
}
