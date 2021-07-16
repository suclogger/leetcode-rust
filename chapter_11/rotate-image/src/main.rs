fn main() {
    let mut v = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    rotate(&mut v);
    assert_eq!(v, vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]);
}
/**
如果允许引入新矩阵的话是很简单的，直接遍历旧矩阵依次填充到新矩阵即可
要求原地转换的话需要一点线性代数的经验
已知矩阵的转置有属性：T'[j][i] = T[i][j]
在转置的基础上达成题目要求的翻转还需要对每个 j : T''[i][m - j] = T'[i][j]
**/
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    for i in 0..m {
        for j in i..m {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
    for i in 0..m {
        for j in 0..m/2 {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[i][m - 1 - j];
            matrix[i][m - 1 - j] = tmp;
        }
    }
}