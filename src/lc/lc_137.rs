/// 找出单独的数
/// 本质是在统计每个比特位上有多少个1
/// 即对于其他数出现2次，统计每一位上1的个数再做模2运算（等价于xor）
/// 同理，对于其他数出现3次，需统计每一位上1的个数再做模3运算
/// 模3运算需要3个状态位,（a, b）：00，01，02
/// 00 -> 01 -> 10 -> 00 -> ...
/// ```rust
/// a = a ^ 1;
/// b = b ^ 1;
///
/// 当 (a|b) == 0时，把0赋值给a，否则((a|b) == 1)可以执行异或操作。写成代码就是a = (a^1) & (a|b)
/// 当 a == 1 时，把0赋值给b，否则(~a == 1)可以执行异或操作。写成代码就是b = (b^1) & ~a
/// ```
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for x in nums {
        b = (b ^ x) & !a;
        a = (a ^ x) & !b;
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_137::single_number;

    #[test]
    fn test_single_number_1() {
        assert_eq!(3, single_number(vec![2, 2, 3, 2]));
        assert_eq!(99, single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}
