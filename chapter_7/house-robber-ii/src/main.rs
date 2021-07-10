use std::cmp::max;

fn main() {
    assert_eq!(rob(vec![2,3,2]), 3);
    assert_eq!(rob(vec![1,2,3,1]), 4);
    assert_eq!(rob(vec![0]), 0);
    assert_eq!(rob(vec![1]), 0);
}

/**
打家劫舍的升级版：房屋成为环形

第一种：要抢最后一家
dp1[i] = max(
    dp[i-2] + nums[i] &&  -- 要抢当前这家(需要考虑环形)
    dp[i-1]
) for i in 2..nums.len()


第二种：不抢最后一家
dp1[i] = max(
    dp[i-2] + nums[i] &&  -- 要抢当前这家(需要考虑环形)
    dp[i-1]
) for i in 1..nums.len()-1

max(dp1[nums.len()], dp2[nums.len -1]);

**/
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }

    let mut dp1 = vec![0; nums.len() + 2];
    let mut dp2 = vec![0; nums.len() + 2];

    for i in 3..nums.len()+2 {
        dp1[i] = max(
            dp1[i-2] + nums[i-2],
            dp1[i-1]
        )
    }
    for i in 2..nums.len()+1 {
        dp2[i] = max(
            dp2[i-2] + nums[i-2],
            dp2[i-1]
        )
    }
    max(dp1[nums.len() + 1], dp2[nums.len() ])
}