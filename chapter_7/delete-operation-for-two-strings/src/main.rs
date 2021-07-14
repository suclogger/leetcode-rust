use std::cmp::min;

fn main() {
    assert_eq!(min_distance(String::from("sea"), String::from("eat")), 2);
}
/**
字符串编辑类DP题目
题目问什么，就作为dp的状态

dp[i][j] : word1处理到i，word2处理到j，使字符串相等所需的最小删除次数

考虑比较i,j所在字符
 A A B C i
 B C j

dp[i][j] =min(
    dp[i- 1][j - 1] if word[i] == word[j]
    min(dp[i][j - 1], dp[i- 1][j]) + 1  if word[i] != word[j]
)

**/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![i32::max_value(); word2.len() + 1]; word1.len() + 1];
    for i in 0..=word1.len() {
        dp[i][0] = i as i32;
    }
    for j in 0..=word2.len() {
        dp[0][j] = j as i32;
    }
    for i in 1..=word1.len() {
        for j in 1..=word2.len() {
            if word1.chars().nth(i - 1).unwrap() == word2.chars().nth( j - 1).unwrap() {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = min(dp[i][j-1], dp[i-1][j]) + 1;
            }

        }
    }
    dp[word1.len()][word2.len()]
}