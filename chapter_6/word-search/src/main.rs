fn main() {
    let board =
        vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
    assert!(exist(board.to_vec(), String::from("ABCCED")));
    assert!(exist(board.to_vec(), String::from("SEE")));
    assert!(!exist(board.to_vec(), String::from("ABCB")));
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let char_vec: Vec<char> = word.chars().collect();
    let mut visited: Vec<Vec<bool>> = vec![vec![false;board[0].len()];board.len()];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if backtracking(&board, &char_vec, &mut visited, i as i32, j as i32, 0) {
                return true;
            }
        }
    }
    false
}

fn backtracking(board: &Vec<Vec<char>>, char_vec: &Vec<char>, visited: &mut Vec<Vec<bool>>, r: i32, c: i32, pos: usize) -> bool{
    if pos >= char_vec.len() {
        return true;
    }
    if r < 0 || c < 0 || r >= board.len() as i32 || c >= board[0].len() as i32 {
        return false;
    }
    if visited[r as usize][c as usize] || board[r as usize][c as usize] != char_vec[pos] {
        return false;
    }
    visited[r as usize][c as usize] = true;
    let direction = vec![-1, 0, 1, 0, -1];
    for d in 0..4 {
        let x = r + direction[d];
        let y = c + direction[d+1];
        if backtracking(board, char_vec, visited, x, y, pos + 1) {
            return true;
        }
    }
    visited[r as usize][c as usize] = false;
    false
}