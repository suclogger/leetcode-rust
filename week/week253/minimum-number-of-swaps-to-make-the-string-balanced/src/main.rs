fn main() {
    println!("Hello, world!");
}


pub fn min_swaps(s: String) -> i32 {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == ']' && !stack.is_empty() {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    return if stack.is_empty() {
        0
    } else {
        stack.len() as i32 / 2
    }
}