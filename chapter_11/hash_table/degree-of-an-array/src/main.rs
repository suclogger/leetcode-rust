fn main() {
    assert_eq!(find_shortest_sub_array(vec![8,9,8,9,8]), 5);
    assert_eq!(find_shortest_sub_array(vec![2,1,1,2,1,3,3,3,1,3,1,3,2]), 7);
}
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hash_table  = std::collections::HashMap::new();
    let mut max_value = (0, 0);
    for (pos, num) in nums.iter().enumerate() {
        let entry = hash_table.entry(num).or_insert((0, pos, pos));
        *entry = (entry.0 + 1, entry.1, pos);
        if entry.0 > max_value.0 || (entry.0 == max_value.0 && entry.2 - entry.1 + 1 < max_value.1) {
            max_value = (entry.0, entry.2 - entry.1 + 1);
        }
    }
    max_value.1 as i32
}