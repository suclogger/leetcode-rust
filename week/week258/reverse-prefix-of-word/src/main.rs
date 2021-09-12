fn main() {
    println!("{}", reverse_prefix(String::from("xyxzxe"), 'z'));
}


pub fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(idx) = word.find(ch) {
        let chars: Vec<char> = word.chars().collect();
        let mut pre_vec = Vec::new();
        pre_vec.append(&mut chars[0..=idx].to_vec());
        pre_vec.reverse();
        pre_vec.append(&mut chars[idx+1..word.len()].to_vec());
        pre_vec.iter().collect()
    } else {
        word
    }
}