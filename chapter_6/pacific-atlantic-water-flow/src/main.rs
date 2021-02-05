fn main() {
    let v = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
    assert_eq!(vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]], pacific_atlantic(v));
}

pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.len() == 0 {
        return vec![]
    }
    let m = matrix.len();
    let n = matrix[0].len();

    let mut can_access_p = vec![vec![false; n]; m];
    let mut can_access_a = vec![vec![false; n]; m];

    let mut ans: Vec<Vec<i32>> = Vec::new();

    for i in 0..m {
        dfs(&matrix, &mut can_access_p, i, 0);
        dfs(&matrix, &mut can_access_a, i, n - 1);
    }

    for j in 0..n {
        dfs(&matrix, &mut can_access_p, 0, j);
        dfs(&matrix, &mut can_access_a, m - 1, j);
    }

    for i in 0..m {
        for j in 0..n {
            if can_access_p[i][j] && can_access_a[i][j] {
                ans.push(vec![i as i32, j as i32]);
            }
        }
    }
    ans
}

fn dfs(matrix: &Vec<Vec<i32>>, can_access: &mut Vec<Vec<bool>>, r: usize, c: usize) {
    if can_access[r][c] {
        return;
    }
    can_access[r][c] = true;
    let direction: Vec<i32> = vec![-1, 0, 1, 0, -1];
    for d in 0..4 {
        let x: i32= r as i32 + direction[d];
        let y: i32= c as i32 + direction[d+1];

        if x>=0 && x< matrix.len() as i32
            && y >=0 && y< matrix[0].len() as i32
                && matrix[x as usize][y as usize] >= matrix[r][c]{
            dfs(matrix, can_access, x as usize, y as usize);
        }
    }
}