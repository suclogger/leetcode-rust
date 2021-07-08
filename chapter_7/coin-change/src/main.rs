fn main() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
    assert_eq!(coin_change(vec![1], 0), 0);
    assert_eq!(coin_change(vec![1], 1), 1);
    assert_eq!(coin_change(vec![1], 2), 2);
    assert_eq!(coin_change(vec![1,2147483647], 2), 2);
}
/**
因为硬币的数量不限，所以是典型的DP完全背包问题：
集合中元素可选多个（或不选），有最小代价约束（最大金额C），求最优解
先遍历容量，再遍历物品。突破口是最后一件物品，不在乎物品一共用过几次：dp[c] = max{dp[c - cost[i]] + val[i]} for i in ...
dp[amount] = min(dp[amount - coins[i]] + 1) for i in 0..coins.len()

**/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![i32::max_value() - 1; amount as usize+ 1];
    dp[0] = 0;
    for c in 1..=amount as usize {
        for i in 0..coins.len() {
            let cur = coins[i] as usize;
            if c < cur {
                dp[c] = std::cmp::min(i32::max_value() - 1, dp[c]);
            } else {
                dp[c] = std::cmp::min(dp[c - cur] +1, dp[c]);
            }
        }
    }
    return if dp[amount as usize] == i32::max_value() - 1 { -1 } else { dp[amount as usize] }
}