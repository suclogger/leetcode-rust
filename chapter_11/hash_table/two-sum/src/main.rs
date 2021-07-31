fn main() {
    assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans = vec![];
    let mut map = std::collections::HashMap::new();
    for (pos, num) in nums.iter().enumerate() {
        let remain = target - *num;
        if let Some(val) = map.get(&remain) {
            ans.push(*val as i32);
            ans.push(pos as i32);
        } else {
            map.insert(*num, pos);
        }
    }
    ans
}