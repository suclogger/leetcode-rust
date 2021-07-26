use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


pub fn are_occurrences_equal(s: String) -> bool {
    let mut char_map = HashMap::new();
    for c in s.chars() {
        let mut count = char_map.entry(c).or_insert(0);
        *count = *count + 1;
    }

    let mut count = 0;
    for (key, value) in &char_map {
        if count == 0 {
            count = *value;
        } else {
            if count != value {
                return false;
            }
        }
    }
    true
}