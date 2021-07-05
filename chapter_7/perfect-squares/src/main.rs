use std::cmp::min;

fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
// 对于分割类型题，动态规划的状态转移方程通常并不依赖相邻的位置，而是依赖于满足分割条件的位置。
pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![i32::max_value() - 1; n as usize + 1];
    dp[0] = 0;
    for i in 1..=n as usize {
        let mut j: usize = 1;
        while j * j <= i as usize{
            dp[i] = min(dp[i], dp[i - j * j] + 1);
            j = j + 1;
        }
    }
    dp[n as usize]
}