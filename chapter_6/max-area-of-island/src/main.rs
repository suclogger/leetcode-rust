fn main() {
    let v = vec![vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
         vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
         vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
         vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
         vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
         vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
         vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
         vec![0,0,0,0,0,0,0,1,1,0,0,0,0]];
    assert_eq!(6, max_area_of_island(v));
}

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut max_area = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                max_area = std::cmp::max(max_area, dfs(&mut grid, i as i32, j as i32));
            }
        }
    }
    max_area as i32
}

fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> usize {
    let direction_vec: Vec<i32> = vec![-1, 0, 1, 0, -1];
    if grid[i as usize][j as usize] == 0 { return 0; }
    // 这行很重要
    grid[i as usize][j as usize] = 0;
    let mut max_area = 1;
    for d in 0..4 {
        let i = i + direction_vec[d];
        let j = j + direction_vec[d + 1];
        if i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32 {
            max_area += dfs(grid, i, j);
        }
    }
    max_area
}

