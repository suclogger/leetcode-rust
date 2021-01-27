fn main() {
    let v = vec![5,7,7,8,8,10];
    assert_eq!(vec![3,4], search_range(v, 8));
    let v = vec![5,7,7,8,8,10];
    assert_eq!(vec![-1,-1], search_range(v, 6));
    let v = vec![1];
    assert_eq!(vec![-1,-1], search_range(v, 0));
    let v = vec![1];
    assert_eq!(vec![0,0], search_range(v, 1));
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1,-1]
    }
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let mid = l + (r - l)/2;
        if nums[mid] >= target {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    let less_r = r;
    l = r;
    r = nums.len();
    while l < r {
        let mid = l + (r - l)/2;
        if nums[mid] > target {
            r = mid;
        } else {
            l = mid+1;
        }
    }
    let large_l = if r > 0 { r - 1 } else {0};
    if less_r == nums.len() || nums[less_r] != target {
        return vec![-1,-1]
    }
    vec![less_r as i32, large_l as i32]
}