
fn main() {
    let s = String::from("ababcbacadefegdehijhklij");
    assert_eq!(partition_labels(s), vec![9,7,8])

}

struct Pre {
    first_occur: usize,
    last_occur: usize,
}

pub fn partition_labels(s: String) -> Vec<i32> {
    let mut m = std::collections::HashMap::new();
    for char_index in s.char_indices() {
        let mut pre = m.entry(char_index.1).or_insert(Pre{
            first_occur: char_index.0,
            last_occur: 0,
        });
        pre.last_occur+=char_index.0;
    }

    let mut res : Vec<i32> = Vec::new();
    let mut ranges:Vec<&Pre> = m.values().collect();
    ranges.sort_by(|a,b|a.first_occur.cmp(&b.first_occur));
    let mut pre_l = ranges[0].first_occur;
    let mut pre_r = ranges[0].last_occur;

    for range in ranges {
        if range.first_occur > pre_r {
            res.push((pre_r - pre_l + 1) as i32);
            pre_l = range.first_occur;
            pre_l = range.last_occur;
        } else {
            pre_r = std::cmp::max(range.last_occur, pre_r);
        }
    }
    res
}