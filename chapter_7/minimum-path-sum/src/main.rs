use std::cmp::min;

fn main() {
    assert_eq!(min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]), 7);
    assert_eq!(min_path_sum(vec![vec![1,2,3],vec![4,5,6]]), 12);
}

// 二维DP问题
// 正向递归的思路是从终点取min(up,left)，然后逐层递归
// 自底向上的DP思路是 dp[x][y] = min((dp[x-1][y] + grid[x][y]),
//                                   (dp[x][y-1] + grid[x][y]));
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 || grid[0].len() == 0 {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    for x in 0..m {
        for y in 0..n {
            if x == 0 && y == 0 {
                dp[x][y] = grid[0][0];
            } else if x ==0 {
                dp[x][y] = dp[x][y-1] + grid[x][y];
            } else if y == 0 {
                dp[x][y] = dp[x-1][y] + grid[x][y];
            } else {
                dp[x][y] = min(
                    dp[x-1][y] + grid[x][y],
                    dp[x][y-1] + grid[x][y]
                )
            }
        }
    }
    dp[m-1][n-1]
}