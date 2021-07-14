use std::cmp::max;

fn main() {
    assert_eq!(wiggle_max_length(vec![0,0,0]), 1);
    assert_eq!(wiggle_max_length(vec![1,7,4,9,2,5]), 6);
    assert_eq!(wiggle_max_length(vec![3,3,3,2,5]), 3);
    assert_eq!(wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]), 7);
    assert_eq!(wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]), 7);
}
/**
子序列类DP问题
因为要考虑摆动，所以需要考虑向上向下两个方向，定义双状态的dp数组
up[i]表示最后一位是向上情况下的最长摆动子数组长度
down[i]表示最后一位是向下情况下的最长摆动子数组长度
考察第i位
if nums[i] > nums[i-1] up[i] = down[i-1] + 1
else if nums[i] < nums[i-1] down[i] = up[i-1] + 1
else up[i] = up[i-1], down[i] = down[i-1]
考察边界
up[0] = 1;
down[0] = 1;
最后取 max(up[nums.len()], down[nums.len()])

观察到up和down都只依赖上一位，所以可以进一步压缩成两个变量

**/
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut up = vec![1; nums.len()];
    let mut down = vec![1; nums.len()];
    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            up[i] = down[i - 1] + 1;
            down[i] = down[i - 1];
        }  else if nums[i] < nums[i - 1] {
            down[i] = up[i - 1] + 1;
            up[i] = up[i - 1];
        }  else {
            up[i] = up[i - 1];
            down[i] = down[i - 1];
        }
    }
    std::cmp::max(up[nums.len() - 1], down[nums.len() - 1])
}