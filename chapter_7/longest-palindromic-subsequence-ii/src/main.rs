fn main() {
    assert_eq!(longest_palindrome_subseq(String::from("bbabab")), 4);
    assert_eq!(longest_palindrome_subseq(String::from("dcbccacdb")), 4);
}

/**
构建三维DP数组
dp[i][j][0]表示从i到j，以字符c开始和结尾的的最长好回文子串长度
dp[i][j][1]表示从i到j，以字符c开始和结尾的的最长好回文子串长度的起止字符


**/

pub fn longest_palindrome_subseq(s: String) -> i32 {
    let chars : Vec<char> = s.chars().collect();

    let mut dp = vec![vec![vec![0; 2]; s.len()]; s.len()];

    for len in 2..=s.len() {
        // i + len - 1 < s.len()
        for i in 0..(s.len() - len + 1) {
            let j = i + len - 1;
            if chars[i] == chars[j] && dp[i + 1][j - 1][1] != chars[i] as u8 {
                dp[i][j][0] = dp[i + 1][j - 1][0] + 2;
                dp[i][j][1] = chars[i] as u8;
            } else {
                if dp[i + 1][j][0] > dp[i][j - 1][0] {
                    dp[i][j][0] = dp[i + 1][j][0];
                    dp[i][j][1] = dp[i + 1][j][1];
                } else {
                    dp[i][j][0] = dp[i][j - 1][0];
                    dp[i][j][1] = dp[i][j - 1][1];
                }
            }
        }

    }
    dp[0][s.len()-1][0] as i32
}