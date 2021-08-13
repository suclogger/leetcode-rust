fn main() {
    println!("Hello, world!");
}

pub fn longest_palindrome(s: String) -> i32 {
    let s_chars : Vec<char> = s.chars().collect();
    let mut table = std::collections::HashMap::new();
    for c in s_chars {
        *table.entry(c).or_insert(0) += 1;
    }
    let mut sin = false;
    let mut ans = s.len();
    for (k, v) in table.iter() {
        if v % 2 == 1 {
            if !sin {
                sin = true;
            } else {
                ans -= 1;
            }
        }
    }
    ans as i32
}