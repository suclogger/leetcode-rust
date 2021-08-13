fn main() {
    // assert_eq!(length_of_longest_substring(String::from(" ")), 1);
    // assert_eq!(length_of_longest_substring(String::from("au")), 2);
    // assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    // assert_eq!(length_of_longest_substring(String::from("abba")), 2);
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut table = std::collections::HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    let mut pre = 0;
    let mut max = 0;
    for (pos, c) in s_chars.iter().enumerate() {
        if let Some(v) = table.get(&c) {
            if pre <= *v {
                max = std::cmp::max(pos - pre, max);
                pre = *v + 1;
            }
        }
        table.insert(c, pos);
    }
    max = std::cmp::max(s.len() - pre, max);
    max as i32
}