fn main() {
    println!("Hello, world!");
}


struct NumMatrix {
    integral: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut integral = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 1..=matrix.len() {
            for j in 1..=matrix[0].len() {
                integral[i][j] = - integral[i- 1][j - 1] + integral[i][j - 1]
                    + integral[i - 1][j] + matrix[i - 1][j - 1];
            }
        }
        NumMatrix {
            integral
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if row1 < 0 || row2 < 0 || col1 < 0 || col2 < 0 ||
            row1 > self.integral.len() as i32 - 2 ||
            row2 > self.integral.len() as i32 - 2 ||
            col1 > self.integral[0].len() as i32 - 2 ||
            col2 > self.integral[0].len() as i32 - 2 ||
            row1 > row2 ||
            col1 > col2 {
            return 0;
        }
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        self.integral[row2 + 1][col2 + 1]
            - self.integral[row1][col2 + 1]
            - self.integral[row2 + 1][col1]
            + self.integral[row1][col1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 *