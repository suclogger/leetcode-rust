use std::cmp::{max};

fn main() {
    assert_eq!(find_longest_chain(vec![vec![1,2], vec![2,3], vec![3,4]]), 2);
}

/**
dp[i] 代表取前i个数对，最长数对链的长度
dp方程：
考察第i个数对
dp[i] = max(
    // 如果要用这个数对
    dp[j] + 1, for j in 0..=i pairs[j][1] < pairs[i][0]
    // 如果不用这个数对
    dp[i - 1]
)
**/
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by(|a,b| a[1].cmp(&b[1]));
    let mut dp = vec![1; pairs.len() + 1];
    for i in 1..=pairs.len() {
        for j in (1..i).rev() {
            if pairs[j- 1][1] < pairs[i - 1][0] {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
        dp[i] = max(dp[i], dp[i - 1]);
    }
    dp[pairs.len()]
}