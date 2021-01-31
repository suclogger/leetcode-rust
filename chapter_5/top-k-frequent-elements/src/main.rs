fn main() {
    assert_eq!(vec![1,2], top_k_frequent(vec![1,1,1,2,2,3], 2));
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for num in nums {
        let v = map.entry(num).or_insert(0);
        *v += 1;
    }
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x,y| y.1.cmp(&x.1));
    let r : Vec<i32> = v.into_iter().map(|x|x.0).collect();
    r[0..(k as usize)].to_vec()
}