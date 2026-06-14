pub fn predict_party_victory(senate: String) -> String {
    let chs: Vec<char> = senate.chars().collect();
    let len = chs.len();
    let (mut rq, mut dq) = (std::collections::VecDeque::new(), std::collections::VecDeque::new());
    for (i, ch) in chs.iter().enumerate() {
        if *ch == 'R' {
            rq.push_back(i);
        } else {
            dq.push_back(i);
        }
    }
    while !rq.is_empty() && !dq.is_empty() {
        let r = rq.pop_front().unwrap();
        let d = dq.pop_front().unwrap();
        if r < d {
            rq.push_back(r + len);
        } else {
            dq.push_back(d + len);
        }
    }
    if rq.is_empty() {
        String::from("Dire")
    } else {
        String::from("Radiant")
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_649::predict_party_victory;

    #[test]
    fn test_predict_party_victory_1() {
        assert_eq!(
            String::from("Dire"),
            predict_party_victory(String::from("RDD"))
        );
    }
}
