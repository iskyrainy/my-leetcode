use std::collections::HashMap;

pub fn invoice_match(bills: Vec<i32>, x: i32) -> (bool, Vec<i32>) {
    let total: i32 = bills.iter().sum();
    if x < 0 || x > total {
        return (false, vec![]);
    }
    if x == 0 {
        return (true, vec![]);
    }

    let mut dp: HashMap<i32, (i32, usize)> = HashMap::new();
    dp.insert(0, (0, 0));

    for (idx, &bill) in bills.iter().enumerate() {
        let current_sums: Vec<i32> = dp.keys().cloned().collect();
        for s in current_sums {
            let new_s = s + bill;
            if new_s <= x && !dp.contains_key(&new_s) {
                dp.insert(new_s, (s, idx));
                if new_s == x {
                    break;
                }
            }
        }
        if dp.contains_key(&x) {
            break;
        }
    }

    if !dp.contains_key(&x) {
        return (false, vec![]);
    }

    let mut chosen = Vec::new();
    let mut cur = x;
    while cur != 0 {
        let (prev, idx) = dp[&cur];
        chosen.push(bills[idx]);
        cur = prev;
    }
    (true, chosen)
}

#[cfg(test)]
mod test {
    use crate::problems::invoice_match::invoice_match;

    #[test]
    fn test_invoice_match_1() {
        assert_eq!(
            (true, vec![100, 100, 10, 10, 5, 5, 5, 5]),
            invoice_match(vec![5, 5, 5, 5, 10, 10, 10, 10, 20, 100, 100, 200], 240)
        );
        panic!()
    }
}
