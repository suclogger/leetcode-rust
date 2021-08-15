fn main() {
    // assert_eq!(rearrange_array(vec![1,2,3,4,5]), vec![1,2,4,5,3]);
    // assert_eq!(rearrange_array(vec![1,2,3]), vec![1,3,2]);
    assert_eq!(rearrange_array(vec![1,5,2,6,3,7,4,8]), vec![1,3,2]);
}

/**
backtracking
**/
pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    fn back_track(nums: &mut Vec<i32>, idx: usize, ans: &mut Vec<Vec<i32>>) {
        if idx == nums.len() {
            ans.push(nums.to_vec());
            return;
        }
        if !ans.is_empty() {
            return;
        }
        for j in idx..nums.len() {
            nums.swap(idx, j);
            if idx == 0 || idx == nums.len() - 1 || ((nums[idx - 1] as f32) + (nums[idx + 1] as f32)) / 2f32 != (nums[idx] as f32) {
                back_track(nums, idx + 1, ans);
            }
            nums.swap(idx, j);
        }
    }
    let mut nums = nums;
    let mut ans = Vec::new();
    back_track(&mut nums, 0, &mut ans);
    ans[0].to_vec()
}