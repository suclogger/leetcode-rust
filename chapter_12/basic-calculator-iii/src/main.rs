fn main() {
    assert_eq!(calculate(String::from("2*(5+5*2)/3+(6/2+8)")), 21);
    assert_eq!(calculate(String::from("(2+6*3+5-(3*14/7+2)*5)+3")), -12);
    assert_eq!(calculate(String::from("2-(5-6)")), 3);
}

pub fn calculate(s: String) -> i32 {
    pub fn calculate_basic(s: String) -> i32 {
        let mut pre = Vec::new();
        let mut stack = std::collections::VecDeque::new();
        let mut op_stack = std::collections::VecDeque::new();
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.push('|');
        let mut negtive = false;
        for c in s_chars {
            if let Some(_) = c.to_digit(10) {
                pre.push(c);
            } else if c == ' ' {
                continue;
            } else {
                let pre_str: String = pre.to_vec().iter().collect();
                let pre_num = if pre_str.is_empty() {
                    negtive = true;
                    continue;
                } else {
                    let n = pre_str.parse::<i32>().unwrap();
                    if negtive { negtive = false; -n } else { n }
                };

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
    let mut stack = Vec::new();
    let mut cur_str = String::new();
    for c in s.chars().into_iter() {
        if c == '(' {
            stack.push(cur_str.to_string());
            cur_str = String::new();
        } else if c == ')' {
            let val = calculate_basic(cur_str.to_string());
            cur_str = stack.pop().unwrap();
            cur_str.push_str(&val.to_string());
        } else {
            cur_str.push(c);
        }
    }
    calculate_basic(cur_str)
}