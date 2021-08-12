fn main() {
    assert_eq!(shortest_palindrome(String::from("aacecaaa")), String::from("aaacecaaa"));
}

/**
这是一道巧妙的KMP的应用
在字符串前拼接字符使之成为回文字符串，意味着字符串可以表达为:
    A'AB
翻转后表达为
    B'A'A
注意到原字符串的前缀跟翻转字符串的后缀匹配
所以这道题目可以转换为
s = B'A'A
p = A'AB
的经典KMP题型（028）

最终得到的结果，就是 s[s.len() - dp[n-1] + 1..s.len()].rev().collect() + s


aaacecaa
 aacecaaa
**/

pub fn shortest_palindrome(s: String) -> String {
    if s.len() == 0 {
        return s;
    }
    //先构造p的后缀数组，在这里p就是原字符串数组
    let mut p_chars : Vec<char> = s.chars().collect();
    let mut suffix = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j = suffix[i - 1];
        while j > 0 && p_chars[j] != p_chars[i] {
            j = suffix[j - 1];
        }
        suffix[i] = j + if p_chars[j] == p_chars[i] {1} else {0};
    }

    // 再构造dp
    let mut dp = vec![0; s.len()];
    let mut s_chars : Vec<char> = s.chars().into_iter().rev().collect();
    dp[0] = if p_chars[0] == s_chars[0] {1} else {0};
    for i in 1..s_chars.len() {
        let mut j = dp[i - 1];
        while j > 0 && s_chars[i] != p_chars[j] {
            j = suffix[j - 1];
        }
        dp[i] = j + if s_chars[i] == p_chars[j] {1} else {0};
    }

    s_chars.append(&mut p_chars[dp[s.len()-1]..s.len()].to_vec());
    s_chars.iter().collect()
}