use std::cmp::max;

fn main() {
    assert_eq!(longest_common_subsequence(String::from("abcde"), String::from("ace")), 3);
}
// 子序列的另一个种dp方程形式：dp数组的第i个值表示到i为止的最优解，而非以i结尾的最优解
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    if text1.len() == 0 || text2.len() == 0 {
        return 0;
    }
    let mut dp = vec![vec![0; text2.len()+1]; text1.len()+1];
    for i in 1..=text1.len() {
        for j in 1..=text2.len() {
            if text1.chars().nth(i-1).unwrap() == text2.chars().nth(j-1).unwrap() {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = max(dp[i][j-1], dp[i-1][j]);
            }
        }
    }
    dp[text1.len()][text2.len()]
}