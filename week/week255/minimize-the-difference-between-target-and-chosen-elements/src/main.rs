use std::borrow::Borrow;

fn main() {
    // assert_eq!(minimize_the_difference(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 13), 0);
    // assert_eq!(minimize_the_difference(vec![vec![1,2,3]], 100), 94);
    assert_eq!(minimize_the_difference(vec![vec![10,3,7,7,9,6,9,8,9,5],vec![1,1,6,8,6,7,7,9,3,9],vec![3,4,4,1,3,6,3,3,9,9],vec![6,9,9,3,8,7,9,6,10,6]],
                                       5), 3);
}

/**
正向类似01背包的思路，通过"和空间"压缩"组合空间"
再根据题意的target约束进一步压缩

**/
pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    let mut set = std::collections::HashSet::new();
    set.insert(0);
    for i in 0..m {
        let mut cur_set = std::collections::HashSet::new();
        let mut nearest_eight_hundred = -1;
        for j in 0..n {
            for pre in set.borrow().into_iter() {
                let cur_sum = pre + mat[i][j];
                if cur_sum > target {
                    if nearest_eight_hundred == -1 ||  cur_sum < nearest_eight_hundred {
                        nearest_eight_hundred = cur_sum;
                        cur_set.insert(cur_sum);
                    }
                } else {
                    cur_set.insert(cur_sum);
                }
            }
        }
        set = cur_set;
    }
    let mut ans = i32::max_value();
    for sum in set {
        ans = std::cmp::min(ans, (target - sum).abs());
    }
    ans
}