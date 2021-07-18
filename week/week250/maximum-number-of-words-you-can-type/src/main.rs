
fn main() {
    assert_eq!(can_be_typed_words(String::from("leet code"), String::from("lt")), 1);
}

pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut ans = 0;
    let bl_set: std::collections::HashSet<char> = broken_letters.chars().into_iter().collect();
    for str in text.split_whitespace().into_iter() {
        if !str.chars().any(|a|bl_set.contains(&a)) {
            ans = ans + 1;
        }
    }
    ans
}