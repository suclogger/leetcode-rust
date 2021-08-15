fn main() {
    assert_eq!(longest_palindrome(String::from("babad")), String::from("bab"));
    assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
    assert_eq!(longest_palindrome(String::from("a")), String::from("a"));
    assert_eq!(longest_palindrome(String::from("ac")), String::from("a"));
    assert_eq!(longest_palindrome(String::from("bb")), String::from("bb"));
    assert_eq!(longest_palindrome(String::from("aacabdkacaa")), String::from("aca"));
    assert_eq!(longest_palindrome(String::from("abcda")), String::from("a"));
}
/**
区间DP题目
先设定长度，再遍历头部

**/


pub fn longest_palindrome(s: String) -> String {
    let s_chars : Vec<char> = s.chars().collect();
    let mut dp = vec![vec![0; s.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][i] = 1;
    }
    let mut ans_i = 0;
    let mut max_len = 0;

    for len in 2..=s.len() {
        for i in 0..(s.len() - len + 1) {
            let j = i + len - 1;
            if s_chars[i] == s_chars[j] &&
                (len - 2 == dp[i + 1][j - 1]) {
                dp[i][j] = dp[i + 1][j - 1] + 2;
                if dp[i][j] > max_len {
                    ans_i = i;
                    max_len = dp[i][j];
                }
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }
    s_chars[ans_i..(ans_i+dp[0][s.len() - 1])].iter().collect()
}