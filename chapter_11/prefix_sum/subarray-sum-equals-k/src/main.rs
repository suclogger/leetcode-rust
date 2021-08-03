fn main() {
    assert_eq!(subarray_sum(vec![1,1,1], 2), 2);
    assert_eq!(subarray_sum(vec![1], 0), 0);
}

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert(0, 1);
    let mut ans = 0;
    let mut pre_sum = 0;
    for num in nums {
        pre_sum = pre_sum + num;
        if let Some(cnt) = hash_map.get(&(pre_sum - k)) {
            ans = ans + cnt;
        }
        let entry = hash_map.entry(pre_sum).or_insert(0);
        *entry = *entry + 1;
    }
    ans
}