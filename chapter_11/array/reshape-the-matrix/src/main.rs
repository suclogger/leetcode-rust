fn main() {
    println!("Hello, world!");
}

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    if (m * n) as i32 != r * c {
        return mat;
    }
    let mut i = 0;
    let mut j: i32 = -1;
    let mut ans = vec![vec![0; c as usize]; r as usize];
    for a in 0..m {
        for b in 0..n {
            j = j + 1;
            if j == c {
                j = 0;
                i = i + 1;
            }
            ans[i][j as usize] = mat[a][b];
        }
    }
    ans
}
