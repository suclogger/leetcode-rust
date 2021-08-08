fn main() {
    println!("Hello, world!");
}

pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
    let mut ns = String::new();
    let mut idx = 0;
    while ns.len() < s.len() && idx < words.len(){
        ns.push_str(&words[idx]);
        idx += 1;
    }
    ns.eq(&s)
}