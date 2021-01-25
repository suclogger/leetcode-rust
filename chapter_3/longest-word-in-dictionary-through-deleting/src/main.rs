fn main() {
    let s = String::from("abpcplea"); // aabcepppl
    let v = vec![String::from("ale"),String::from("apple"),String::from("monkey"),String::from("plea")];
    assert_eq!(find_longest_word(s, v), "apple");

    let s = String::from("abpcplea");
    let v = vec![String::from("a"),String::from("b"),String::from("c")];
    assert_eq!(find_longest_word(s, v), "a");

    let s = String::from("bab");
    let v = vec![String::from("ba"),String::from("ab"),String::from("a")];
    assert_eq!(find_longest_word(s, v), "ab");
}

pub fn find_longest_word(s: String, d: Vec<String>) -> String {
    let mut max_len = 0;
    let mut str = String::from("");
    let l_v : Vec<char> = s.chars().collect();
    for dic in d {
        if dic.len() < max_len || (dic.len() == max_len && dic > str) {
            continue;
        }
        let mut l = 0;
        let mut r = 0;
        let r_v : Vec<char> = dic.chars().collect();
        while l < s.len() {
            if r == dic.len() {
                break;
            }
            if l_v[l] != r_v[r] {
                l += 1;
            } else {
                l += 1;
                r += 1;
            }
        }
        if r == dic.len() && (max_len < dic.len() || (max_len == dic.len() && dic < str)){
            max_len = dic.len();
            str = dic;
        }
    }
    return str;
}