use std::cmp::min;

fn main() {
    assert_eq!(min_steps(3), 3);
}
/**
DP-字符串类问题
dp数组每个元素表示得到长度为i的字符串需要的最小操作次数
dp[i] = min(dp[h] + dp[i/h]) for h in 1..i

**/

pub fn min_steps(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    let mut dp = vec![n; n as usize + 1];
    for i in 2..=n as usize {
        dp[i] = i as i32;
        for h in 1..=i {
            if i % h == 0 {
                dp[i] = min(dp[i], dp[h] + dp[i/h]);
            }
        }
    }
    dp[n as usize]
}