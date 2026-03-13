use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.len() == 2 {
        return max(nums[0], nums[1]);
    }
    let mut res = vec![0; nums.len()];
    for (i, &num) in nums.iter().enumerate() {
        if i < 2 {
            res[i] = num;
            continue;
        } else if i == 2 {
            res[i] = num + res[i - 2];
            continue;
        }
        res[i] = max(res[i - 3], res[i - 2]) + num;
    }
    max(res[nums.len() - 1], res[nums.len() - 2])
}

#[cfg(test)]
mod tests {
    use crate::lc::lc_198::rob;

    #[test]
    fn test_rob_1() {
        assert_eq!(22, rob(vec![2, 1, 15, 7, 1, 5]));
    }
}
