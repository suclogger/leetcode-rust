fn main() {
    println!("Hello, world!");
}

pub fn make_fancy_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut ans = Vec::new();
    for (pos, c) in chars.to_vec().iter().enumerate() {
        if pos < 2 {
            ans.push(*c);
            continue;
        }
        if *c == chars[pos - 1] && *c == chars[pos - 2] {
            chars[pos - 2] = '*';
        } else {
            ans.push(*c);
        }
    }
    ans.iter().collect()
}