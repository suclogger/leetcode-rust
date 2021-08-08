use std::collections::BTreeMap;

fn main() {
    assert_eq!(longest_obstacle_course_at_each_position(vec![5,1,5,5,1,3,4,5,1,4]),
    vec![1,1,2,3,2,3,4,5,3,5]
    );
}

/**
是个容易误导为单调栈题型的动态规划题目

dp[i]是以第i个元素结尾的最大长度
dp[i] = max(dp[j]) + 1 for j in 0..i && nums[j] <= nums[i]

**/

pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    let mut dp  = vec![1;obstacles.len()];
    let mut tm: BTreeMap<i32, i32> = std::collections::BTreeMap::new();
    for i in 0..obstacles.len() {
        if let Some((&k, &v)) = tm.range(1..=obstacles[i]).into_iter()
            .max_by(|&a, &b|a.1.cmp(b.1)) {
            let n = v+1;
            tm.insert(obstacles[i], n);
            dp[i] = n;
        } else {
            tm.insert(obstacles[i], 1);
        }
    }
    dp
}