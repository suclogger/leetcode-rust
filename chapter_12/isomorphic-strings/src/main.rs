fn main() {
    println!("Hello, world!");
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut dic_table = std::collections::HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut map_set = std::collections::HashSet::new();
    for idx in 0..s.len() {
        if let Some(&exist_map) = dic_table.get(&s_chars[idx]) {
            if exist_map != t_chars[idx] {
                return false;
            }
        } else {
            if map_set.contains(&t_chars[idx]) {
                return false;
            }
            dic_table.insert(s_chars[idx], t_chars[idx]);
            map_set.insert(t_chars[idx]);
        }
    }
    true
}