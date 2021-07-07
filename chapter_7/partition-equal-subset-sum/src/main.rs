fn main() {
    assert_eq!(can_partition(vec![1,5,11,5]), true);

}

/**
需要做一层转换，把分割问题转化为背包问题：能否分割成等值的两部分 =》能否取部分值和为总和的一半.
方程：dp[i][sum] = dp[i-1][sum] || dp[i-1][sum-num[i]]
还要注意边界条件，遍历的范围等细节
**/
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    let sum = sum  as usize;
    if sum % 2 != 0 {
        return false;
    }

    let mut dp = vec![vec![false; sum/2 + 1]; nums.len() + 1];
    dp[0][0] = true;
    for i in 1..=nums.len() {
        for s in 0usize..=sum/2 {
            let cur_num = nums[i -1] as usize;
            if s <  cur_num{
                dp[i][s] = dp[i-1][s];
            } else {
                dp[i][s] = dp[i-1][s] || dp[i-1][s - cur_num];
            }
        }
    }
    dp[nums.len()][sum/2 as usize]
}