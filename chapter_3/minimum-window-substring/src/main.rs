fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    assert_eq!("BANC", min_window(s, t));
}

pub fn min_window(s: String, t: String) -> String {
    let mut t_freq = vec![0; 128];
    let mut win_freq = vec![0; 128];
    for c in t.chars() {
        match t_freq.get(c as usize) {
            Some(freq) => {
                t_freq[c as usize] = freq + 1;
            },
            None => {
                t_freq[c as usize] = 1;
            }
        }
    }
    let s_char_array: Vec<char> = s.chars().collect();

    let mut min_l = 0;
    let mut min_length = s.len() + 1;

    // window 中包含t字符的个数，相等时说明全部包含
    let mut distance = 0;

    let mut left = 0;
    let mut right = 0;
    while right < s.len() {
        if t_freq[s_char_array[right] as usize] == 0 {
            right +=1;
            continue;
        }

        match win_freq.get(s_char_array[right] as usize) {
            Some(freq) => {
                if *freq < t_freq[s_char_array[right] as usize] {
                    distance+=1;
                }
            },
            None => {}
        }

        match win_freq.get(s_char_array[right] as usize) {
            Some(freq) => {
                win_freq[s_char_array[right]  as usize] = freq + 1;
            },
            None => {
                win_freq[s_char_array[right]  as usize] = 1;
            }
        }
        right +=1 ;

        while distance == t.len() {
            if right - left < min_length {
                min_length = right - left;
                min_l = left;
            }

            if t_freq[s_char_array[left] as usize] == 0 {
                left += 1;
                continue;
            }

            match win_freq.get(s_char_array[left] as usize) {
                Some(freq) => {
                    if *freq == t_freq[s_char_array[left] as usize] {
                        distance-=1;
                    }
                },
                None => {}
            }

            match win_freq.get(s_char_array[left] as usize) {
                Some(freq) => {
                    win_freq[s_char_array[left] as usize] = freq - 1;
                },
                None => { }
            }
            left +=1 ;
        }

    }
    if min_length == s.len() + 1 {
        return String::from("");
    }

    String::from(&s[min_l..(min_l+min_length)])
}
