fn main() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(2147395599), 46339);
}

pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut l = 1;
    let mut r = x;
    while l <= r {
        let mid = l + (r-l)/2;
        let sqrt = x / mid;
        if sqrt == mid {
            return mid;
        } else if mid > sqrt {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    r
}