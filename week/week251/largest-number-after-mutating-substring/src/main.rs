fn main() {
    assert_eq!(maximum_number(String::from("334111"), vec![0,9,2,3,3,2,5,5,5,5]), String::from("334999"))
}

pub fn maximum_number(num: String, change: Vec<i32>) -> String {
    let mut new_vec = Vec::new();
    let mut changed = false;
    let mut pre_changed = true;
    for c in num.chars() {
        let n_idx = c as i32 - 48;
        if (pre_changed || (!pre_changed && !changed)) && change[n_idx as usize] >= n_idx {
            new_vec.push((change[n_idx as usize] as u8 + 48) as char);
            if change[n_idx as usize] == n_idx {
                if changed {
                    pre_changed = true;
                }
            } else {
                changed = true;
                pre_changed = true;
            }

        } else {
            pre_changed = false;
            new_vec.push(c);
        }
    }
    new_vec.iter().collect()
}