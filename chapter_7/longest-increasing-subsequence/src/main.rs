use std::cmp::max;

fn main() {
    // assert_eq!(4, length_of_lis(vec![10,9,2,5,3,7,101,18]));
    // assert_eq!(4, length_of_lis(vec![0,1,0,3,2,3]));
    // assert_eq!(1, length_of_lis(vec![7,7,7,7,7,7,7]));
    // assert_eq!(3, length_of_lis(vec![4,10,4,3,8,9]));
    assert_eq!(6, length_of_lis(vec![1,3,6,7,9,4,10,5,6]));
}
// 简单的DP双循环，定义DP数组，dp[i]记录以i结尾的最大子递增数组长度，方程是dp[i] = max(dp[j..i-1])
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let mut max_len = 1;
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = max(dp[j] + 1, dp[i])
            }
        }
        max_len = max(dp[i], max_len);
    }
    max_len
}