fn main() {
    assert_eq!(get_lucky(String::from("leetcode"), 2), 6);
}

pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut chars: Vec<char>= Vec::new();
    let mut s: String = s;
    let mut sum = 0;
    for _i in 0..=k {
        if sum != 0 {
            s = sum.to_string();
            sum = 0;
        } else if !chars.is_empty(){
            s = String::from(&chars.into_iter().collect::<String>());
            chars = Vec::new();
        }
        for c in s.chars() {
            let num = c as i32;
            // num
            if num >= 48 && num <= 57 {
                sum = sum + num - 48;
            } else {
                chars.extend(&mut (num - 96).to_string().chars())
            }
        }
    }
    sum
}