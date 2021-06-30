fn main() {
    let mut board = vec![vec!['5','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']];
    solve_sudoku(&mut board);
    assert_eq!(board,
    vec![vec!['5','3','4','6','7','8','9','1','2'],vec!['6','7','2','1','9','5','3','4','8'],vec!['1','9','8','3','4','2','5','6','7'],vec!['8','5','9','7','6','1','4','2','3'],vec!['4','2','6','8','5','3','7','9','1'],vec!['7','1','3','9','2','4','8','5','6'],vec!['9','6','1','5','3','7','2','8','4'],vec!['2','8','7','4','1','9','6','3','5'],vec!['3','4','5','2','8','6','1','7','9']])
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn dfs(board: &mut Vec<Vec<char>>, mut m: usize, mut n: usize, m_seen: &mut Vec<Vec<bool>>,
           n_seen: &mut Vec<Vec<bool>>, x_seen: &mut Vec<Vec<Vec<bool>>>) -> bool {
        if  n > 8 {
            n = 0;
            m = m + 1;
            if m > 8 {
                return true;
            }
        }
        if board[m][n] == '.' {
            for i in 0..9 {
                if !m_seen[m][i] && !n_seen[n][i] && !x_seen[m/3][n/3][i] {
                    board[m][n] =  ((i as u8 + 1)  + b'0') as char;
                    m_seen[m][i] = true;
                    n_seen[n][i]= true;
                    x_seen[m/3][n/3][i]= true;
                    if dfs(board, m, n + 1, m_seen, n_seen, x_seen) {
                        return true;
                    }
                    m_seen[m][i] = false;
                    n_seen[n][i]= false;
                    x_seen[m/3][n/3][i]= false;
                    board[m][n] = '.';
                }
            }
        } else {
            return dfs(board, m, n + 1, m_seen, n_seen, x_seen);
        }
        false
    }

    let mut m_seen = vec![vec![false; 9]; 9];
    let mut n_seen = vec![vec![false; 9]; 9];
    let mut x_seen = vec![vec![vec![false; 9]; 3]; 3];
    for m in 0..9 {
        for n in 0..9 {
            if board[m][n] != '.' {
                let v = board[m][n].to_digit(10).unwrap() as usize - 1;
                m_seen[m][v] = true;
                n_seen[n][v] = true;
                x_seen[m/3][n/3][v] = true;
            }
        }
    }
    dfs(board, 0, 0, &mut m_seen,&mut n_seen,&mut x_seen);
}