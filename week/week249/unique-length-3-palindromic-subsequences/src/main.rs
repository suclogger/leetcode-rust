fn main() {
    assert_eq!(count_palindromic_subsequence(String::from("aabca")), 3);
    assert_eq!(count_palindromic_subsequence(String::from("bbcbaba")), 4);
    assert_eq!(count_palindromic_subsequence(String::from("uuuuu")), 1);
    assert_eq!(count_palindromic_subsequence(String::from("tlpjzdmtwderpkpmgoyrcxttiheassztncqvnfjeyxxp")), 161);
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    if s.len() < 3 {
        return 0;
    }
    let mut dic = vec![vec![false; 26]; 26];
    let mut ans = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut char_map = std::collections::HashMap::new();
    for i in 0..chars.len() {
        let count = char_map.entry(chars[i]).or_insert(i);
        if *count != i {
            if i - *count >= 2 {
                for j in *count+1..i {
                    let side_char = (chars[i] as u8 - 97) as usize;
                    let middle_char = (chars[j] as u8  - 97) as usize;
                    if !dic[side_char][middle_char] {
                        dic[side_char][middle_char] = true;
                        ans = ans + 1;
                    }
                }
                *count = i - 1;
            }
        }
    }
    ans as i32
}