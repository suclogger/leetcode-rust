fn main() {
    assert_eq!(split_painting())
}

pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
    let mut p_arr = vec![vec![0, 0]; 100000];
    let mut max_idx = 0;
    for seg in segments {
        for i in seg[0]..seg[1] {
            p_arr[i as usize]  = vec![p_arr[i as usize][0] + seg[2], seg[2]];
        }
        if seg[1] > max_idx {
            max_idx = seg[1];
        }
    }
    let mut ans = Vec::new();
    let mut pre = 1;
    for i in 2..=max_idx {
        if p_arr[i as usize] != p_arr[i as usize - 1] {
            ans.push(vec![pre as i64, i as i64, p_arr[i as usize - 1][0] as i64]);
            pre = i;
        }
    }
    ans
}