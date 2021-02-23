fn main() {
    assert_eq!(1, shortest_bridge(vec![vec![0,1],vec![1,0]]));
    assert_eq!(2, shortest_bridge(vec![vec![0,1,0],vec![0,0,0],vec![0,0,1]]));
    assert_eq!(1, shortest_bridge(vec![vec![1,1,1,1,1],vec![1,0,0,0,1],vec![1,0,1,0,1],vec![1,0,0,0,1],vec![1,1,1,1,1]]));
    assert_eq!(1, shortest_bridge(vec![vec![0,1,0,0,0],vec![0,1,0,1,1],vec![0,0,0,0,1],vec![0,0,0,0,0],vec![0,0,0,0,0]]));
}

pub fn shortest_bridge(a: Vec<Vec<i32>>) -> i32 {
    let m = a.len();
    let n = a[0].len();
    let mut a = a;
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut flipped = false;
    for i in 0..a.len() {
        if flipped {
            break;
        }
        for j in 0..a[0].len() {
            if a[i][j] == 1 {
                dfs(&mut points, &mut a, m, n, i, j);
                flipped = true;
                break;
            }
        }
    }

    let mut level = 0;
    let direction: Vec<i32> = vec![-1, 0, 1, 0, -1];
    while points.len() > 0 {
        level += 1;
        let mut cur_level_size = points.len();
        while cur_level_size > 0 {
            cur_level_size -= 1;
            let (x, y) = points.remove(0);
            for i in 0..4 {
                let r : i32 = x as i32 + direction[i];
                let c : i32 = y as i32 + direction[i + 1];
                if r <0 || c<0 || c as usize >= a[0].len() || r as usize >= a.len() {
                    continue;
                }
                let r: usize = r as usize;
                let c: usize = c as usize;
                if a[r][c] == 1 {
                    return level;
                } else if a[r][c] == 2 {
                    continue;
                } else if a[r][c] == 0 {
                    points.push((r, c));
                    a[r][c] = 2;
                }
            }
            a[x][y] = 2;
        }
    }
    0
}

fn dfs(point: &mut Vec<(usize, usize)>, a: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    if i >= m || j >= n || a[i][j] == 2 {
        return;
    }
    if a[i][j] == 0 {
        point.push((i, j));
        return;
    }
    a[i][j] = 2;
    if i > 0 {
        dfs(point, a, m, n, i-1, j);
    }
    dfs(point, a, m, n, i, j+1);
    dfs(point, a, m, n, i+1, j);
    if j > 0 {
        dfs(point, a, m, n, i, j-1);
    }
}