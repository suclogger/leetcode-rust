use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}
/**
因为硬币的数量不限，所以是典型的DP完全背包问题：
集合中元素可选可不选，有最小代价约束（最大金额C），求最优解
定义dp数组 dp[i][m] 表示在前m个元素中，凑成m需要的最少硬币个数
dp[i][m] = max(dp[i-1][m], dp[i-1][m-coin[i]] + 1)
**/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut ans = coins.len() + 1;
    let mut dp = vec![vec![coins.len() + 1; max_sum + 1]; coins.len() + 1];
    for i in 1..=coins.len() {
        for s in 0..=max_sum {
            let cur_v = coins[i] as usize;
            if s < cur_v as usize {
                dp[i][s] = 1;
            } else {
                dp[i][s] = min(dp[i-1][s], dp[i-1][s-cur_v] + 1);
            }
            if s >= amount as usize {
                ans = min(ans, dp[i][s]);
            }
        }
        if ans == coins.len() + 1 { -1 } else { ans }
    }



    0
}