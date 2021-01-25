fn main() {
    let s = String::from("eceba");
    assert_eq!(length_of_longest_substring_k_distinct(s, 2), 3);
    let s = String::from("aa");
    assert_eq!(length_of_longest_substring_k_distinct(s, 1), 2);
}

pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {

    let mut count = 0;
    let mut r = 0;
    let mut l = 0;
    let mut max_len = 0;
    let mut v = vec![0;128];
    let s_v : Vec<char> = s.chars().collect();
    while r < s.len() {
        if v[s_v[r] as usize] >0 {
            v[s_v[r] as usize] += 1;
        } else {
            count += 1;
            v[s_v[r] as usize] = 1;
        }
        r += 1;
        while count > k && l < r {
            if v[s_v[l] as usize] >1 {
                v[s_v[l] as usize] -= 1;
            } else {
                count -= 1;
                v[s_v[l] as usize] = 0;
            }
            l+=1;
        }
        if r - l > max_len {
            max_len = r - l;
        }
    }
    max_len as i32
}