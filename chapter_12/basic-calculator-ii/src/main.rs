fn main() {
    // assert_eq!(calculate(String::from("3+2*2")), 7);
    assert_eq!(calculate(String::from("1-1+1")), 1);
}

/**
考虑使用deque，优先级高的操作符 * / 在尾部操作
优先级低的操作符从头部操作
**/

pub fn calculate(s: String) -> i32 {
    let mut pre = Vec::new();
    let mut stack = std::collections::VecDeque::new();
    let mut op_stack = std::collections::VecDeque::new();
    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars.push('|');
    for c in s_chars {
        if let Some(_) = c.to_digit(10) {
            pre.push(c);
        } else if c == ' ' {
            continue;
        } else {
            let pre_str: String = pre.to_vec().iter().collect();
            let pre_num = pre_str.parse::<i32>().unwrap();

            // 提前运算
            if !op_stack.is_empty() && (op_stack[op_stack.len() - 1] == '*'
                ||  op_stack[op_stack.len() - 1] == '/') {
                let op =  op_stack.pop_back().unwrap();
                let num = stack.pop_back().unwrap();
                if op == '*' {
                    stack.push_back(num * pre_num);
                } else {
                    stack.push_back(num / pre_num);
                }
            } else {
                stack.push_back(pre_num);
            }
            if c != '|' {
                op_stack.push_back(c);
            }
            pre.clear();
        }
    }

    let mut ans = stack.pop_front().unwrap();
    while !op_stack.is_empty() {
        let op = op_stack.pop_front().unwrap();
        if op == '+' {
            ans += stack.pop_front().unwrap();
        } else {
            ans -= stack.pop_front().unwrap();
        }
    }
    ans
}