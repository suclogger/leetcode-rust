use std::cmp::min;

fn main() {
    assert_eq!(update_matrix(vec![vec![0,0,0],
                    vec![0,1,0],
                    vec![0,0,0]]),
               vec![vec![0,0,0],
                    vec![0,1,0],
                    vec![0,0,0]]
    );
    assert_eq!(update_matrix(vec![vec![0,0,0],
                                  vec![0,1,0],
                                  vec![1,1,1]]),
               vec![vec![0,0,0],
                    vec![0,1,0],
                    vec![1,2,1]]
    );
}
// 因为要考虑四个方向，所以要左上一次，右下一次
pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if mat.len() == 0 || mat[0].len() == 0 {
        return vec![];
    }
    let m = mat.len();
    let n = mat[0].len();
    let mut dp = vec![vec![i32::max_value() - 1; n]; m];

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                dp[i][j] = 0;
            } else {
                if i > 0 {
                    dp[i][j] = min(dp[i][j], dp[i - 1][j] + 1);
                }
                if j > 0 {
                    dp[i][j] = min(dp[i][j], dp[i][j - 1] + 1);
                }
            }
        }
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if dp[i][j] != 0 {
                if i < m-1 {
                    dp[i][j] = min(dp[i][j], dp[i + 1][j] + 1);
                }
                if j < n - 1 {
                    dp[i][j] = min(dp[i][j], dp[i][j + 1] + 1);
                }
            }
        }
    }
    dp
}
