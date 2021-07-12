use std::cmp::max;

fn main() {
    assert_eq!(integer_break(10), 36);
}
/**

这道题先不急着直接拆成很碎，先拆成两个部分，各自是最优解，整体也得到了最优解
所以从dp数组构建的思路

dp[i] = max(dp[i], dp[i-j]*dp[j] for j = 1..n-i)

再考虑边界值

dp[2] = 2;
dp[3] = 3;
表示2，3都没必要再拆分了，再拆分只会更小

**/

pub fn integer_break(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if n == 3 {
        return 2;
    }
    let mut dp = vec![0; n as usize + 1];
    dp[2] = 2;
    dp[3] = 3;
    for i in 4..=n as usize {
        for j in 2..=i/2 {
            dp[i] = max(dp[i], dp[i - j] * dp[j]);
        }
    }
    dp[n as usize]

}