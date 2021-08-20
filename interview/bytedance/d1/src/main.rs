fn main() {
    println!("{}", find_longest_sub_str(String::from("SGEfweeeEEabcdddDddd")));
}


fn find_longest_sub_str(source: String) -> String {
    if source.len() <= 1 {
        return source;
    }
    let mut idx = 0;
    let mut cur_idx = 0;
    let mut ans = 1;
    let mut cur = 1;
    let s_chars : Vec<char> = source.to_uppercase().chars().collect();
    for i in 1..source.len() {
        if s_chars[i] == s_chars[i- 1] {
            cur += 1;
        } else {
            if cur > ans {
                ans = cur;
                idx = cur_idx;
            }
            cur = 1;
            cur_idx = i;
        }
        if i == (source.len() - 1) {
            if cur > ans {
                ans = cur;
                idx = cur_idx;
            }
        }
    }
    let origin_chars : Vec<char> = source.chars().collect();
    origin_chars[idx..=(idx+ans-1)].iter().collect()
}