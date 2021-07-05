
fn main() {
    assert_eq!(num_decodings(String::from("12")), 2);
    assert_eq!(num_decodings(String::from("226")), 3);
    assert_eq!(num_decodings(String::from("1123")), 5);
    assert_eq!(num_decodings(String::from("2611055971756562")), 4);
}
// 考察遍历的每个字符，如果这个字符只能作为第一位，dp[i] = dp[i-1]
// 考察遍历的每个字符，如果这个字符既能做第一位又能做第二位，dp[i] = dp[i-1] + dp[i-2]
pub fn num_decodings(s: String) -> i32 {
    if s.starts_with("0") {
        return 0;
    }
    if s.len() <= 1 {
        return s.len() as i32;
    }
    let s_char_array: Vec<char> = s.chars().collect();
    let mut dp = vec![1; s.len()];
    for i in 1..s.len() {
        let cur = s_char_array[i] as u32 - '0' as u32;
        let pre = s_char_array[i-1] as u32 - '0' as u32;
        if i < 2 {
            if (pre == 2 && cur > 0 && cur <7) || (pre ==1 && cur != 0) {
                dp[i] = 2;
            } else if cur == 0 {
                if  pre > 0 && pre < 3 {
                    dp[i] = 1;
                } else {
                    return 0;
                }
            } else {
                dp[i] = 1;
            }
        } else {
            if (pre == 2 && cur > 0 && cur <7) || (pre ==1 && cur != 0) {
                dp[i] = dp[i - 1] + dp[i - 2];
            } else if cur == 0 {
                if  pre > 0 && pre < 3 {
                    dp[i] = dp[i - 2];
                } else {
                    return 0;
                }
            } else {
                dp[i] = dp[i - 1];
            }
        }
    }
    dp[s.len() - 1]
}