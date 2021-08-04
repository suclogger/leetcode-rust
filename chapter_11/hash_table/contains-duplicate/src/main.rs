fn main() {
    println!("Hello, world!");
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash_table = std::collections::HashSet::new();
    for num in nums {
        if hash_table.contains(&num) {
            return true;
        }
        hash_table.insert(num);
    }
    false
}