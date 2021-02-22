fn main() {
    assert_eq!(solve_n_queens(4), vec![vec![".Q..","...Q","Q...","..Q."],vec!["..Q.","Q...","...Q",".Q.."]]);
    assert_eq!(solve_n_queens(1), vec![vec!["Q"]]);
}


pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    // 理论上需要4个访问数组，但是我们按行遍历的话，可以省略行访问数组
    let mut column_visited = vec![false;n as usize];
    // left low right high
    let mut llrh_visited = vec![false;(2*n - 1) as usize];
    let mut lhrl_visited = vec![false;(2*n - 1) as usize];

    let mut ans: Vec<Vec<String>> = Vec::new();
    let mut cur_ans: Vec<String> = Vec::new();
    backtracking(&mut ans, &mut cur_ans, &mut column_visited, &mut llrh_visited, &mut lhrl_visited, 0, n as usize);
    ans
}

fn backtracking(ans: &mut Vec<Vec<String>>, cur_ans: &mut Vec<String>, column_visited: &mut Vec<bool>,
                    llrh_visited: &mut Vec<bool>, lhrl_visited: &mut Vec<bool>,
                        row: usize, n: usize) {
    if row == n {
        ans.push(cur_ans.to_vec());
        return;
    }

    let mut char_vec: Vec<char> = vec!['.'; n];
    for i in 0..n {
        if column_visited[i] || llrh_visited[row + i] || lhrl_visited[n - i + row - 1] {
            continue;
        }
        column_visited[i] = true;
        llrh_visited[row + i] = true;
        lhrl_visited[n - i + row - 1] = true;
        char_vec[i] = 'Q';
        cur_ans.push(char_vec.to_vec().into_iter().collect());
        backtracking(ans, cur_ans, column_visited, llrh_visited, lhrl_visited, row + 1, n);
        char_vec[i] = '.';
        column_visited[i] = false;
        llrh_visited[row + i] = false;
        lhrl_visited[n - i + row - 1] = false;
        cur_ans.pop();
    }

}