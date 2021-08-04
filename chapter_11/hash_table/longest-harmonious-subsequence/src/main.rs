fn main() {
    assert_eq!(find_lhs(vec![1,1,1,1]), 0);
}

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut hash_table = std::collections::HashMap::new();
    let mut max_len = 0;
    for num in nums {
        *(hash_table.entry(num).or_insert(0)) += 1;
    }
    for (k, v) in hash_table.iter() {
        let l_max = if let Some(l) =  hash_table.get(&(k - 1)) {
            v + l
        } else {
            0
        };
        let r_max = if let Some(r) =  hash_table.get(&(k + 1)) {
            v + r
        } else {
            0
        };
        max_len = std::cmp::max(std::cmp::max(l_max, r_max), max_len);
    }
    max_len
}