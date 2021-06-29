fn main() {
    let mut board : Vec<Vec<char>> = vec![vec!['X','X' ,'X' ,'X' ],vec!['X' ,'O' ,'O' ,'X' ],vec!['X','X','O','X'],vec!['X','O','X','X']];
    solve(&mut board);
    assert_eq!(board, vec![vec!['X','X','X','X'],vec!['X','X','X','X'],vec!['X','X','X','X'],vec!['X','O','X','X']]);

}
// 先从最外侧填充，然后再考虑里侧。
pub fn solve(board: &mut Vec<Vec<char>>) {
    if board.len() <= 0 || board[0].len() <= 0 {
        return;
    }

    for i in 0..board.len() {
        dfs(board, i as i32, 0);
        dfs(board, i as i32, board[0].len() as i32  - 1);
    }

    for j in 0..board[0].len() {
        dfs(board, 0, j as i32);
        dfs(board, board.len() as i32 - 1, j as i32);
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'G' {
                board[i][j] = 'O';
            } else if board[i][j] == 'O' {
                board[i][j] = 'X';
            }
        }
    }
}

fn dfs(board: &mut Vec<Vec<char>>, m: i32, n: i32) {
    if m < 0 || n < 0 || m >= board.len() as i32 || n >= board[0].len() as i32 || board[m as usize][n as usize] != 'O' {
        return;
    }

    board[m as usize][n as usize] = 'G';
    let direction = vec![-1, 0, 1, 0, -1];
    for idx in 0..direction.len()-1 {
        dfs(board, m + direction[idx], n + direction[idx+1]);
    }
}
