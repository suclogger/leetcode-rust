fn main() {
    let v = vec![1,2,3];
    assert_eq!(vec![
        vec![1,2,3],
        vec![1,3,2],
        vec![2,1,3],
        vec![2,3,1],
        vec![3,1,2],
        vec![3,2,1]
    ], permute(v));
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut ans: Vec<Vec<i32>> = Vec::new();
    backtrack(&mut nums, 0, &mut ans);
    ans
}

fn backtrack(nums: &mut Vec<i32>, level: usize, ans: &mut Vec<Vec<i32>>) {
    if level == nums.len() {
        ans.push(nums.to_vec());
        return;
    }

    for i in level..nums.len() {
        nums.swap(i, level);
        backtrack(nums, level+1, ans);
        nums.swap(i, level);
    }
}