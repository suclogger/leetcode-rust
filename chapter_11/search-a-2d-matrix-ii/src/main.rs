fn main() {
    // assert_eq!(search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 5), true);
    // assert_eq!(search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 20), false);
    assert_eq!(search_matrix(vec![vec![-5]], -10), false);
}
/**
搜索技巧：从右上开始，小的往左，大的往下
**/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut i = 0;
    let mut j = matrix[0].len() as i32 - 1;
    let mut found = false;
    while !found && i < matrix.len() && j >= 0 {
        if target == matrix[i][j as usize] {
            found = true;
        } else if target < matrix[i][j as usize] {
            j = j - 1;
        } else {
            i = i + 1;
        }
    }
    found
}