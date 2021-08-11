fn main() {
    // assert_eq!(str_str(String::from("abc"), String::from("c")), 2);
    assert_eq!(str_str(String::from("aaa"), String::from("aaa")), 0);
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 {
        return 0;
    }
    let mut h_idx = 0;
    let mut n_idx = 0;
    let mut pre_idx = 0;
    let h_chars: Vec<char> = haystack.chars().collect();
    let n_chars: Vec<char> = needle.chars().collect();
    while h_idx < haystack.len() {
        if h_chars[h_idx] == n_chars[n_idx] {
            if n_idx == needle.len() - 1 {
                return if n_idx > 0 { pre_idx as i32 } else {h_idx as i32};
            }
            if n_idx == 0 {
                pre_idx = h_idx;
            }
            h_idx += 1;
            n_idx += 1;
        } else {
            if n_idx > 0 {
                h_idx = pre_idx + 1;
                n_idx = 0;
            } else {
                h_idx += 1;
            }
        }
    }
    -1
}