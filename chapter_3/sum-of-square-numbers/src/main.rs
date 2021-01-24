fn main() {
    assert!(judge_square_sum(5));
    assert!(judge_square_sum(4));
    assert!(!judge_square_sum(3));
    assert!(judge_square_sum(2));
    assert!(judge_square_sum(1));
}

pub fn judge_square_sum(c: i32) -> bool {
    let mid = (c as f64).sqrt();
    if mid.fract() == 0.0 {
        return true;
    }
    let mut l = 0;
    let mut r = mid as i32;
    while l <= r {
        let tmp = l*l + r * r;
        if tmp == c {
            return true;
        } else if tmp > c {
            r-=1;
        } else {
            l+=1;
        }
    }
    false
}