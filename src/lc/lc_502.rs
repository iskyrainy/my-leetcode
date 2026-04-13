use std::collections::BinaryHeap;

pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut w = w;
    let mut i = 0_usize;
    let mut zip = profits.iter().zip(&capital).collect::<Vec<_>>();
    zip.sort_unstable_by(|&x, &y| (*x.1).cmp(y.1));
    let mut heap = BinaryHeap::new();
    for _ in 0..k {
        unsafe {
            loop {
                if i == zip.len() {
                    break;
                }
                let &(p, c) = zip.get_unchecked(i);
                if *c > w {
                    break;
                }
                heap.push(*p);
                i += 1;
            }
            if let Some(v) = heap.pop() {
                w += v;
            }
        }
    }
    w
}

#[cfg(test)]
mod test {
    use crate::lc::lc_502::find_maximized_capital;

    #[test]
    fn test_find_maximized_capital_1() {
        assert_eq!(
            4,
            find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
        );
    }

    #[test]
    fn test_find_maximized_capital_2() {
        assert_eq!(
            6,
            find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2])
        );
    }
}
