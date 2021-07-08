use std::cmp::min;

fn main() {
    assert_eq!(min_distance(String::from("horse"), String::from("ros")), 3);
}
/**
动态规划-字符串编辑类题型-编辑距离
定义二维数组，dp[m][m]表示第一个字符串取前m位，第二个字符串取n位，所需的最小编辑次数
dp[m][m] =
    if (word[m] == word[n] )
        dp[m-1][n-1]
    else
        min(
            dp[m-1][n-1] + 1,   -- 替换m/n位置
            dp[m-1][n] + 1, -- 插入n位置/删除m位置
            dp[m][n-1] + 1  -- 插入m位置/删除n位置
        )
**/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j as i32;
            } else if j == 0 {
                dp[i][j] = i as i32;
            } else {
                if word1.chars().nth(i - 1).unwrap() == word2.chars().nth(j - 1).unwrap() {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = min(min(dp[i-1][j-1] + 1, dp[i-1][j] + 1), dp[i][j-1] + 1);
                }
            }
        }
    }
    dp[m][n]
}