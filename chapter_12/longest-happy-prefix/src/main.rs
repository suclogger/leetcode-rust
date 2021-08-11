fn main() {
    // assert_eq!(longest_prefix(String::from("level")), String::from("l"));
    assert_eq!(longest_prefix(String::from("ababab")), String::from("abab"));
}
/**
后缀数组
1. 定义 dp[i] = j 表示以i结尾，最长的后缀数组的长度 为 j 即 s[0..j] = s[i - j + 1..i]
2. 参照dp的思路，找到 dp[i] 和 dp[i - 1]的关系

假定有 j = dp[i - 1]

 * * * * * * * * ——————————————————————   * * * * * * * *
 0             j —————————————————————— i-j            i-1

考察 dp[i] 与dp[i - 1]的关系

若 s[i] == s[j + 1] :
dp[i] = j + 1

 *  *  *  *  *  *  *  *  *   —————————————————————   *  *  *  *  *  *  *  *  *
 0          j-1  j j+1       —————————————————————  i-j                  i-1 i


否则，另j' = dp[j - 1]

 +  +  +  ———— +  +  +  ?   —————————————————————  +  +  +  ?
 0     j'           j-1 j   —————————————————————    i-1 i

若  s[i] == s[j' + 1] :
dp[i] = j' + 1;

否则，继续往下找 j'' = dp[j' - 1]
以此类推

**/

pub fn longest_prefix(s: String) -> String {
    let mut dp = vec![0; s.len()];
    let s_chars :Vec<char> = s.chars().collect();
    for i in 1..s.len() {
        let mut j = dp[i - 1];
        while s_chars[j] != s_chars[i] && j > 0 {
            j = dp[j - 1];
        }
        dp[i] = j + if s_chars[j] == s_chars[i] {1} else {0};
    }
    s_chars[0..dp[s.len() - 1]].iter().collect()
}