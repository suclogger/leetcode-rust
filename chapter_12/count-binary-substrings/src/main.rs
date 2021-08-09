fn main() {
    assert_eq!(count_binary_substrings(String::from("00110011")), 6);
}

/**
记录正在遍历的字符数，已经先前遍历的字符的数量，遇到不同字符的时候翻转
**/

pub fn count_binary_substrings(s: String) -> i32 {
    let mut pre = 0;
    let mut cur = 1;
    let mut ans = 0;
    let s_chars: Vec<char> = s.chars().collect();
    for pos in 1..s_chars.len() {
        if s_chars[pos] == s_chars[pos - 1] {
            cur += 1;
        } else {
            pre = cur;
            cur = 1;
        }

        if pre >= cur {
            ans += 1;
        }
    }
    ans
}