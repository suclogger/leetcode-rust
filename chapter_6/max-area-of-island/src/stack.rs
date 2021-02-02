pub fn max_area_of_island_stack(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut max_area = 0;
    let mut island : Vec<(i32,i32)> = Vec::new();
    let direction: Vec<i32> = vec![-1, 0, 1, 0, -1];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 { continue; }
            island.push((i as i32, j as i32));
            let mut area = 0;
            while !island.is_empty() {
                match island.pop() {
                    None => {}
                    Some((i, j)) => {
                        grid[i as usize][j as usize] = 0;
                        area += 1;
                        for d in 0..4 {
                            let i_u = i + direction[d as usize];
                            let j_u = j + direction[(d+1) as usize];

                            if i_u < grid.len() as i32 && i_u >= 0
                                && j_u > 0 && j_u < grid[0].len() as i32 {
                                if grid[i_u as usize][j_u as usize] == 1{
                                    island.push((i_u, j_u));
                                }
                            }
                        }
                    }
                }
            }
            max_area = std::cmp::max(max_area, area);
        }
    }
    max_area
}
