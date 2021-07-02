fn main() {
    assert_eq!(4, rob(vec![1,2,3,1]));
    assert_eq!(12, rob(vec![2,7,9,3,1]));
}

// dp[i] = max(dp[i-1], dp[i-2] + nums[i]);
//
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut pre2 = 0;
    let mut pre1 = nums[0];
    let mut cur = pre1;
    for i in 2..=nums.len() {
        cur = std::cmp::max(pre2 + nums[i - 1], pre1);
        pre2 = pre1;
        pre1 = cur;
    }
    cur
}