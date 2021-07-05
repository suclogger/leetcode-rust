fn main() {
    assert_eq!(word_break(String::from("leetcode"), vec![String::from("leet"), String::from("code")]), true);
    // assert_eq!(word_break(String::from("a"), vec![String::from("a")]), true);
}


pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: std::collections::HashSet<String> = word_dict.into_iter().collect();
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    for i in 1..=s.len() {
        for j in (0..i).rev() {
            if dp[j] && word_set.contains(&s[j..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[s.len()]
}