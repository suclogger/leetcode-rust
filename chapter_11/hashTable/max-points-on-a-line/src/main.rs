fn main() {
    // assert_eq!(max_points(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]]), 4);
    // assert_eq!(max_points(vec![vec![1,0],vec![0,0]]), 2);
    assert_eq!(max_points(vec![vec![0,0],vec![4,5],vec![7,8],vec![8,9],vec![5,6],vec![3,4],vec![1,1]]), 5);
}

/**
每两点可以确认一个斜率，斜率相同，起始点相同，就是同一直线
x相同的需要单独考虑
时间复杂度是O(n^2)

**/

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() < 2 {
        return points.len() as i32;
    }
    let mut max = 0;
    for i in 0..points.len() {
        let mut table = std::collections::HashMap::new();
        let mut same_x = 1;
        let mut max_j = 0;
        for j in (i + 1)..points.len() {
            if points[j][0] == points[i][0] {
                same_x = same_x + 1;
                max_j = std::cmp::max(max_j, same_x);
            } else {
                let ver: i32 =(((points[j][1] - points[i][1]) as f32/(points[j][0] - points[i][0]) as f32) * 100f32).trunc() as i32;
                let entry = table.entry(ver).or_insert(1);
                *entry = *entry + 1;
                max_j = std::cmp::max(max_j, *entry);
            }
        }
        max = std::cmp::max(max_j, max);
    }
    max
}