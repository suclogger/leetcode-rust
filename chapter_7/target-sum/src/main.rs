fn main() {
    // assert_eq!(find_target_sum_ways(vec![1,1,1,1,1], 3), 5);
    // assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    // assert_eq!(find_target_sum_ways(vec![1000], 1000), 1);
    assert_eq!(find_target_sum_ways(vec![2,107,109,113,127,131,137,3,2,3,5,7,11,13,17,19,23,29,47,53], 1000), 1);
}
/**

每个数字可以正着用，负着用，总和有限，选择集有限，暗示是01背包问题
定义动态数组 dp[i][c] 为取前i个元素，消耗总和c情况下的表达式组合数

dp[i][c] =
    dp[i][c] += dp[i-1][c + nums[i]]
    dp[i][c] += dp[i-1][c - nums[i]]


再考察边界，认为dp[0][0] = 1

这里比较麻烦的是负数下标的处理，通过加上最大和转化为正数下标

**/

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let max_sum = nums.iter().sum::<i32>();
    if target > max_sum || target < -max_sum {
        return 0;
    }
    let mut dp = vec![vec![0; max_sum as usize * 2 + 1]; nums.len() + 1];
    dp[0][max_sum as usize] = 1;
    for i in 1..=nums.len() {
        for j in 0-max_sum as i32..=max_sum as i32 {
            if j + nums[i - 1] <= max_sum{
                dp[i][(j + max_sum) as usize] = dp[i][(j + max_sum) as usize] + dp[i-1][(j + nums[i - 1]  + max_sum) as usize];
            }
            if j - nums[i - 1]  + max_sum >= 0 {
                dp[i][(j + max_sum) as usize] = dp[i][(j + max_sum) as usize] + dp[i-1][(j - nums[i - 1]  + max_sum) as usize];
            }
        }
    }
    dp[nums.len()][(target + max_sum) as usize]
}