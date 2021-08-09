fn main() {
    println!("Hello, world!");
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_table = std::collections::HashMap::new();
    let mut t_table = std::collections::HashMap::new();
    for s_char in s.chars() {
        *s_table.entry(s_char).or_insert(0) += 1;
    }
    for t_char in t.chars() {
        *t_table.entry(t_char).or_insert(0) += 1;
    }
    for (&k, &v) in s_table.iter() {
        if let Some(&val) =  t_table.get(&k) {
            if val != v {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}