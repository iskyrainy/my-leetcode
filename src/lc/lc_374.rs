unsafe extern "C" {
    fn guess(n: i32) -> i32;
}

unsafe fn guessNumber(n: i32) -> i32 {
    if unsafe { guess(n) } == 0 {
        return n;
    }
    let mut lower: usize = 1;
    let mut higher = n as usize;
    while lower < higher {
        let mid: i32 = ((lower + higher) >> 1) as i32;
        if unsafe { guess(mid) } == 0 {
            return mid;
        } else if unsafe { guess(mid) } == 1 {
            lower = mid as usize;
        } else {
            higher = mid as usize;
        }
    }
    0
}

#[cfg(test)]
mod test {

    #[test]
    fn test_guess_number_1() {}
}
