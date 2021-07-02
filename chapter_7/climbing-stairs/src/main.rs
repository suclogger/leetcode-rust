fn main() {
   assert_eq!(2, climb_stairs(2));
}
// 一维动态规划，状态转移方程是 dp[i] = dp[i-1] + dp[i-2]
pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    let mut pre1 = 1;
    let mut pre2 = 2;
    let mut cur = 2;
    for _ in 3..=n {
        cur = pre1 + pre2;
        pre1 = pre2;
        pre2 = cur;
    }
    cur
}