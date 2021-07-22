fn main() {
    println!("Hello, world!");
}

/**
还记得大明湖畔的编译原理里面的确定性下推状态机的例子吗，举的就是括号识别的例子

**/

pub fn is_valid(s: String) -> bool {
    let mut char_stack = Vec::new();
    for c in s.chars() {
        if c == ')' {
            if char_stack.is_empty() || char_stack.pop().unwrap() != '(' {
                return false;
            }
        } else if c == ']' {
            if char_stack.is_empty() || char_stack.pop().unwrap() != '[' {
                return false;
            }
        } else if c == '}' {
            if char_stack.is_empty() || char_stack.pop().unwrap() != '{' {
                return false;
            }
        } else {
            char_stack.push(c);
        }
    }
    char_stack.is_empty()
}