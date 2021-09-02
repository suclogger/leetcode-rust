fn main() {
    occupy(vec![10,9,8,7,4,5,6,7,2,8], 40);
}

// 动态规划，定义二维数组dp[i][s] 表示以前i个库位结尾的，达到s库存的所需的最小库位数
// 对于每个i和每个s的转移方程是
// dp[i][s] =  min(dp[i - 1][t - vec[i]] + 1, dp[i - 1][t - vec[i]])
//
fn occupy(invs: Vec<usize>, target: usize) {
    let mut dp_val = vec![vec![Vec::new(); target + 1]; invs.len() + 1];
    let mut dp = vec![vec![usize::max_value() - 1; target + 1]; invs.len() + 1];
    for i in 0..=invs.len() {
        dp[i][0] = 0;
    }

    for i in 1..=invs.len() {
        for s in 1..=target {
            // 2 记录选择
            let choose = if s <= invs[i - 1] {1} else {dp[i-1][s - invs[i-1]] + 1};
            let not_choose = dp[i-1][s];
            if choose <= not_choose {
                if choose == 1 {
                    dp_val[i][s].push(invs[i - 1]);
                } else {
                    let mut v = dp_val[i-1][s- invs[i-1]].to_vec();
                    v.push(invs[i - 1]);
                    dp_val[i][s] = v;
                }
            } else {
                dp_val[i][s] = dp_val[i-1][s].to_vec();
            }

            // 1 初步验证
            dp[i][s] = std::cmp::min(
                // 选他
                if s <= invs[i - 1] {1} else {dp[i-1][s - invs[i-1]] + 1},
                // 不选他
                dp[i-1][s]
            );
            //println!("dp i = {}  s = {} val = {}", i, s, dp[i][s]);
        }
    }
    println!("{}", dp[invs.len()][target]);

    println!("------- choose list ------");

    let val_vec = dp_val[invs.len()][target].to_vec();
    for val in  val_vec{
        println!("{}", val);
    }
}