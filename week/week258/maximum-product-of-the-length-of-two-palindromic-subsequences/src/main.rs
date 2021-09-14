fn main() {
    assert_eq!(max_product(String::from("leetcodecom")), 9);
}

/**

有两个要点：
    最长回文子串 -> DP区间解法
    整个字符串选取元素考虑通过bitmap遍历拆分
**/

pub fn max_product(s: String) -> i32 {
    fn lps(s: &str, bitmap: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut cur_char = Vec::new();
        for i in 0..s.len() {
            if (1 << i & bitmap) == 1 << i {
                cur_char.push(chars[i]);
            }
        }

        let mut dp = vec![vec![0;cur_char.len()]; cur_char.len()];
        // 边界
        for i in 0..cur_char.len() {
            dp[i][i] = 1;
        }

        // 外层是长度
        for len in 2..=cur_char.len() {
            // 内层是起始点 i + len - 1 < cur_char.len() => i < cur_char.len() - len + 1
            for i in 0..(cur_char.len() - len + 1) {
                let j = i + len - 1;
                if cur_char[i] == cur_char[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[0][cur_char.len() - 1]
    }

    let mut ans = 0;
    for i in 1..(1 << s.len()) - 1 {
        ans = std::cmp::max(ans, lps(&s, i) * lps(&s, (1 << s.len()) - 1 - i));
    }
    ans
}