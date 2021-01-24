fn main() {
    assert!(valid_palindrome(String::from("aba")));
    assert!(valid_palindrome(String::from("abca")));
    assert!(!valid_palindrome(String::from("abc")));
    assert!(valid_palindrome(String::from("eccer")));
    assert!(valid_palindrome(String::from("ebcbbececabbacecbbcbe")));
}
fn valid_palindrome_inner(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }
    let mut l = 0;
    let mut r = s.len()-1;
    let s_arr : Vec<char> = s.chars().collect();
    while l <= r && r > 0 {
        if s_arr[l] == s_arr[r] {
            l += 1;
            r -= 1;
            continue;
        }
        return false;
    }
    true
}

pub fn valid_palindrome(s: String) -> bool {
    if s.len() < 2 {
        return true;
    }
    let mut l = 0;
    let mut r = s.len()-1;
    let s_arr : Vec<char> = s.chars().collect();
    while l <= r && r > 0 {
        if s_arr[l] == s_arr[r] {
            l += 1;
            r -= 1;
            continue;
        }
        // leetcode 上提交需要用Self::指定同类方法
        // return Self::valid_palindrome_inner(&s[l..r]) || Self::valid_palindrome_inner(&s[l+1..r+1]);
        return valid_palindrome_inner(&s[l..r]) || valid_palindrome_inner(&s[l+1..r+1]);
    }
    true
}
