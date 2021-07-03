fn main() {
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
}
// 一维DP进阶
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {

    if nums.len() < 3 {
        return 0;
    }

    let mut dp = vec![0; nums.len()];
    for i in 2..nums.len() {
        if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
            dp[i] = dp[i - 1] + 1;
        }
    }
    let sum: usize = dp.iter().sum();
    sum as i32
}