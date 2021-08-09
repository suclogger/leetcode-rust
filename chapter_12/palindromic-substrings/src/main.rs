fn main() {
    assert_eq!(count_substrings(String::from("abc")) ,3);
}

/**

可以从中轴快速判断回文

**/

pub fn count_substrings(s: String) -> i32 {
    fn count_inner(chars: &Vec<char>, l: usize, r: usize) -> i32 {
        let mut l = l as i32;
        let mut r = r;
        let mut ans = 0;
        while l >= 0 && r < chars.len() && chars[l as usize] == chars[r] {
            l-=1;
            r+=1;
            ans+=1;
        }
        ans
    }
    let mut ans = 0;
    let chars: Vec<char> = s.chars().collect();
    for idx in 0..s.len() {
        ans += count_inner(&chars, idx, idx);
        ans += count_inner(&chars, idx, idx + 1);
    }
    ans
}