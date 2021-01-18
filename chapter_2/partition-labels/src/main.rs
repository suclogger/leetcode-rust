fn main() {
    let s = String::from("ababcbacadefegdehijhklij");
    assert_eq!(partition_labels(s), vec![9,7,8])

}

pub fn partition_labels(s: String) -> Vec<i32> {
    if s.len() < 1 {
        return vec![]
    }
    let mut m = std::collections::HashMap::new();
    for char_index in s.char_indices() {
        let last_idx = m.entry(char_index.1).or_insert(char_index.0);
        *last_idx = char_index.0;
    }
    let mut res = Vec::new();
    let mut cur_idx = 0;
    let mut next_idx = 0;
    for char_index in s.char_indices() {
        match m.get(&char_index.1) {
            Some(last_idx) => {
                if &char_index.0 >= &next_idx && char_index.0 >= *last_idx {
                    res.push((char_index.0 - cur_idx + 1) as i32);
                    cur_idx = char_index.0 + 1;
                    next_idx = char_index.0 + 1;
                } else {
                    next_idx = std::cmp::max(*last_idx, next_idx);
                }
            },
            None => { }
        }
    }
    res
}