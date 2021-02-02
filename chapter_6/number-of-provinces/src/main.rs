fn main() {
    let v = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
    assert_eq!(find_circle_num(v), 2);
    let v = vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]];
    assert_eq!(find_circle_num(v), 3);
}

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut circle = 0;
    let mut visited: Vec<bool> = vec![false;is_connected.len()];
    for i in 0..is_connected.len() {
        if !visited[i] {
            dfs(&is_connected, i, &mut visited);
            circle += 1;
        }
    }
    circle
}

fn dfs(is_connected: &Vec<Vec<i32>>, i: usize, visited: &mut Vec<bool>) {
    visited[i] = true;
    let len = is_connected.len();
    for j in 0..len {
        if is_connected[i][j] == 1 && !visited[j] {
            dfs(is_connected, j, visited);
        }
    }
}
