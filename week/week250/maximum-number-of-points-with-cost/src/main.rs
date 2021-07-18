fn main() {
    assert_eq!(max_points(vec![vec![1,2,3],vec![1,5,1],vec![3,1,1]]), 9);
    // assert_eq!(max_points(vec![vec![2,4,0,5,5],vec![0,5,4,2,5],vec![2,0,2,3,1],vec![3,0,5,5,2]]), 17);
    // assert_eq!(max_points(vec![vec![4,4,2,2,1],vec![5,5,2,1,2],vec![3,1,5,5,2],vec![3,2,0,0,3]]), 15);
}

pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        // 表明第i行选第j个值，最大的答案
        let mut dp = vec![0;points[0].len()];
        for i in 0..points.len() {
            let mut cur_dp = vec![0;points[0].len()];
            for j in 0..points[0].len() {
                if i == 0 {
                    cur_dp[j] = points[i][j] as i64;
                } else {
                    for pre_j in 0..points[0].len() {
                        cur_dp[j] = std::cmp::max(cur_dp[j],
                                                 dp[pre_j] + points[i][j]  as i64 - (pre_j  as i64 - j   as i64).abs())
                    }
                }
            }
            dp = cur_dp;
        }
        dp.to_vec().into_iter().max().unwrap()
}
