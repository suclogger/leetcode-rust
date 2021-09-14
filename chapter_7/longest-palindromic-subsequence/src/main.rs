fn main() {
    assert_eq!(longest_palindrome_subseq(String::from("cbbd")), 2);
    assert_eq!(longest_palindrome_subseq(String::from("bbbab")), 4);
}

/**
区间dp
**/
pub fn longest_palindrome_subseq(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut dp = vec![vec![0; s.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][i] = 1;
    }
    for len in 2..=s.len() {
        // i + len - 1 < s.len()
        for i in 0..(s.len() - len + 1) {
            let j = i + len - 1;
            if chars[i] == chars[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[0][s.len() - 1]
}