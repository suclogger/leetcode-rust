use std::cmp::max;

fn main() {
    assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
}

/**
一维DP问题

dp[i] 表示取前第i位，最大子数组的和

考察是否添加当前第i位

dp[i] = max(
    dp[i-1] + nums[i], // if dp[i-1] > 0
    nums[i]             // if dp[i-1] < 0
)
有余力可以做下空间压缩
**/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() < 2 {
        return nums[0];
    }
    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    for i in 1..nums.len() {
        dp[i] = max(nums[i], dp[i-1] + nums[i]);
    }
    *dp.iter().max().unwrap()
}