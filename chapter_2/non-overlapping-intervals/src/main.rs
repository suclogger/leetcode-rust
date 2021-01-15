fn main() {
    let v = vec![vec![1,2], vec![2,3]];
    assert_eq!(erase_overlap_intervals(v), 0);
}

pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.len()<=0 {
        return 0;
    }
    let mut intervals = intervals;
    intervals.sort_by(|a,b| a[1].cmp(&b[1]));
    let mut erase_count = 0;

    let mut prev_end = intervals[0][1];
    for i in 1..intervals.len() {
        if intervals[i][0] >= prev_end {
            prev_end = intervals[i][1];
        } else {
            erase_count+=1;
        }
    }
    erase_count
}
