pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.partition_point(|&x| x < target) as _
}

#[cfg(test)]
mod test {
    use crate::lc::lc_035::search_insert;

    #[test]
    fn test_search_insert_1() {
        assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
    }
    #[test]
    fn test_search_insert_2() {
        assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
    }
    #[test]
    fn test_search_insert_3() {
        assert_eq!(0, search_insert(vec![1, 3, 5, 6], -1));
    }
}
