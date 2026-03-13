/// 给你一个整数数组 coins ，表示不同面额的硬币；
/// 以及一个整数 amount ，表示总金额。
/// 计算并返回可以凑成总金额所需的最少的硬币个数 。
/// 如果没有任何一种硬币组合能组成总金额，返回 -1 。
/// 你可以认为每种硬币的数量是无限的。
/// dp[amount] = min(dp[amount - coins.0], dp[amount - coins.1], ...) + 1
/// dp[coins.0] = dp[coins.1] = ... = dp[coins.len()-1] = 1
/// for i from 0..coins.len if !coins.contains(i) dp[i] = -MAX else dp[i] = 1
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount as usize) + 1];
    dp[0] = 0;
    
    for &c in coins.iter() {
        if c > amount || c < 0 { continue; }
        let c: usize = c as usize;
        for j in c..=amount as usize {
            dp[j] = std::cmp::min(dp[j], dp[j - c] + 1);
        }
    }
    
    if dp[amount as usize] > amount { return -1; }
    dp[amount as usize]
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_322::coin_change;

    #[test]
    fn test_coin_change_1() {
        assert_eq!(3, coin_change(vec![1, 2, 5], 11));
    }

    #[test]
    fn test_coin_change_2() {
        assert_eq!(-1, coin_change(vec![2], 3));
    }

    #[test]
    fn test_coin_change_3() {
        assert_eq!(2, coin_change(vec![1,2,5], 10));
    }
}
