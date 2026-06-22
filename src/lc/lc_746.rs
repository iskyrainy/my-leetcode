pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n + 1];
    for i in 2..=n {
        dp[i] = (dp[i - 2] + cost[i - 2]).min(dp[i - 1] + cost[i - 1]);
    }
    dp[n]
}

#[cfg(test)]
mod test {
    use crate::lc::lc_746::min_cost_climbing_stairs;

    #[test]
    fn test_min_cost_climbing_stairs_1() {
        assert_eq!(15, min_cost_climbing_stairs(vec![10, 15, 20]));
    }
}
