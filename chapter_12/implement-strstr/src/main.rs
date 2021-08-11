fn main() {
    assert_eq!(str_str(String::from("abc"), String::from("c")), 2);
    assert_eq!(str_str(String::from("aaa"), String::from("aaa")), 0);
}

/**
KMP算法
s: haystack，范围字符串
p: needle，查找的字符串

1. 先对p构建后缀数组 suffix[i] = k 表示最大长度的 p[0..k] = p[i-k+1..i] ，构建方法参考 longest-happy-prefix
2. 对s构建KMP数组 dp[i] = k 表示最大长度的 s[i-k+1..i] = p[0..k-1] , 这里就打通了s和k
考察 dp[i] 和 dp[i-1] 的关系
另 j = dp[i-1]

s数组
—————————————   * * * * * * * *  ？ —————————————
————————————— i-j            i-1 i  —————————————

p数组
                * * * * * * * *  ？ —————————————
                0             j

若 s[i] == p[j + 1] : dp[i] = j + 1
否则 j = suffix[j - 1]
然后依次类推

**/
pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() == 0 {
        return if needle.len() == 0 {0} else {-1};
    }
    if needle.len() == 0 {
        return 0;
    }
    let mut suffix = vec![0; needle.len()];
    let n_chars : Vec<char> = needle.chars().collect();
    for i in 1..needle.len() {
        let mut j = suffix[i - 1];
        while n_chars[j] != n_chars[i] && j > 0 {
            j = suffix[j - 1];
        }
        suffix[i] = j + if n_chars[j] == n_chars[i] {1} else {0};
    }

    let s_chars: Vec<char> = haystack.chars().collect();
    let mut dp = vec![0;haystack.len()];
    dp[0] = if s_chars[0] == n_chars[0] {1} else {0};
    if dp[0] == 1 && needle.len() == 1 {
        return 0;
    }

    for i in 1..s_chars.len() {
        let mut j = dp[i - 1];
        while s_chars[i] != n_chars[j] && j > 0{
            j = suffix[j - 1];
        }
        dp[i] = j + if s_chars[i] == n_chars[j] {1} else {0};
        if dp[i] == needle.len() {
            return (i - needle.len() + 1) as i32;
        }
    }
    -1
}