use std::collections::HashSet;

fn main() {
    assert_eq!(longest_consecutive(vec![100,4,200,1,3,2]), 4);
    assert_eq!(longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut max_len = 0;
    let mut table: HashSet<i32> = nums.into_iter().collect();

    while !table.is_empty() {
        let random = *table.iter().next().unwrap();
        table.remove(&random);
        let mut before = random;
        let mut after = random;
        while table.remove(&(before - 1)) {
            before = before - 1;
        }
        while table.remove(&(after + 1)) {
            after = after + 1;
        }
        max_len = std::cmp::max(max_len, after - before + 1);
    }
    max_len
}