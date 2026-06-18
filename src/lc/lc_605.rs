pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let len = flowerbed.len();
    for i in 0..len {
        if flowerbed[i] == 0 {
            let mut f = true;
            if i + 1 < len && flowerbed[i + 1] == 1 {
                f = false;
            }
            if i >= 1 && flowerbed[i - 1] == 1 {
                f = false;
            }
            if f {
                flowerbed[i] = 1;
                n -= 1;
            }
        }
    }
    n <= 0
}

#[cfg(test)]
mod test {
    use crate::lc::lc_605::can_place_flowers;

    #[test]
    fn test_can_place_flowers_1() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }
}
