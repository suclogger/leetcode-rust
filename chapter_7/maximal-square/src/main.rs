use std::cmp::{min, max};

fn main() {
    assert_eq!(maximal_square(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']]),
    4);
}

// 定义dp数组的每个值是以当前坐标为右下角的最大正方形边长
// dp方程：dp[i][j] = min(dp[i-1][j-1], dp[i][j-1], dp[i-1][j]) + 1
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }
    let mut max_side = 0;
    let m =  matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                if matrix[i][j] == '1' {
                    dp[i][j] = 1;
                }
            } else {
                if matrix[i][j] == '1' {
                    dp[i][j] = min(min(dp[i-1][j-1], dp[i-1][j]), dp[i][j-1]) + 1;
                }
            }
            max_side = max(max_side, dp[i][j]);
        }
    }

    max_side * max_side
}