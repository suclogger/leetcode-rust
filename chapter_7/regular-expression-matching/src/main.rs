fn main() {
    // assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
    assert_eq!(is_match(String::from("ab"), String::from(".*")), false);
}
/**
DP双字符串问题
定义dp数组，dp[i][j]表示 dp[0:i] 能否匹配dp[0:j]
接下来考虑怎么往下面靠
dp[i-1][j-1]
dp[][j-1]
dp[i-1][j]
一般从最后一位开始考虑
s: X X X X X i
p: Y Y Y Y j
结构上考虑怎么划分做到MECE
**/

pub fn is_match(s: String, p: String) -> bool {

    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[0][0] = true;
    for j in 2..=p.len() {
        dp[0][j] = p.chars().nth(j - 1).unwrap() == '*' && dp[0][j - 2];
    }

    for i in 1..=s.len() {
        for j in 1..=p.len() {
            let cur_s = s.chars().nth(i - 1).unwrap();
            let cur_p = p.chars().nth(j - 1).unwrap();
            if cur_p != '.' && cur_p != '*' {
                dp[i][j] = cur_s == cur_p && dp[i-1][j-1];
            } else if cur_p == '.' {
                dp[i][j] = dp[i-1][j-1];
            } else {
                // *
                dp[i][j] =
                    // zero pre_char
                    dp[i][j-2] ||
                    // one or more pre_char
                    ((cur_s ==  p.chars().nth(j - 2).unwrap() || p.chars().nth(j - 2).unwrap() == '.') && dp[i-1][j])
            }
        }
    }
    dp[s.len()][p.len()]
}