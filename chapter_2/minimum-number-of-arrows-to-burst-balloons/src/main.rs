fn main() {
    let points = vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]];
    assert_eq!(find_min_arrow_shots(points), 2);
    let points = vec![vec![3,9],vec![7,12],vec![3,8],vec![6,8],vec![9,10],vec![2,9],vec![0,9],vec![3,9],vec![0,6],vec![2,8]];
    assert_eq!(find_min_arrow_shots(points), 2);
    let points = vec![vec![9,12],vec![1,10],vec![4,11],vec![8,12],vec![3,9],vec![6,9],vec![6,7]];
    assert_eq!(find_min_arrow_shots(points), 2);
}

pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    if points.len() <1 {
        return 0;
    }
    let mut points = points;
    points.sort_by(|a,b|if a[0]==b[0] { a[1].cmp(&b[1]) } else { a[0].cmp(&b[0]) });
    let mut arrow_counts = 1;
    let mut prev = points[0][1];
    for i in 1..points.len() {
        if points[i][0] <= prev && points[i][1] >=prev {
            continue;
        }
        if points[i][0] > prev {
            arrow_counts +=1;
            prev = points[i][1];
        } else {
            prev = points[i][1];
        }
    }
    arrow_counts
}