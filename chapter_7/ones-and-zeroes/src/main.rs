use std::cmp::max;

fn main() {
    // assert_eq!(find_max_form(vec![String::from("10"), String::from("0001"), String::from("111001"), String::from("1"),String::from( "0")], 5, 3), 4);
    assert_eq!(find_max_form(vec![String::from("10"), String::from("0"), String::from("1")], 1, 1), 2);
}
/**
典型的DP背包问题：
最大代价约束m和n，数组中的项可选可不选，求最大项
定义dp数组，dp[i][x][y]代表到第i项，x个0，y个1的最大子数组长度
**/
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut max_len = 0;

    let mut dp = vec![vec![vec![0; n+1]; m+1]; strs.len() + 1];

    for i in 1..strs.len() + 1 {
        for x in 0..=m {
            for y in 0..=n {
                let zero_count = strs[i - 1].matches('0').count();
                let one_count = strs[i - 1].matches('1').count();
                dp[i][x][y] = max(dp[i - 1][x][y], if x<zero_count || y<one_count {0} else {dp[i-1][x - zero_count][y - one_count]  + 1});
                max_len = max(max_len, dp[i][x][y]);
            }
        }
    }
    max_len
}