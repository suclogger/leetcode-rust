fn main() {
    let v = vec![4,2,3];
    assert!(check_possibility(v));
    let v = vec![4,2,1];
    assert!(!check_possibility(v));
    let v = vec![5,7,1,8];
    assert!(check_possibility(v));
    let v = vec![-1,4,2,3];
    assert!(check_possibility(v));
}

pub fn check_possibility(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return true;
    }
    let mut nums = nums;
    let mut met = false;
    for i in 1..nums.len() {
        if nums[i] < nums[i-1] {
            if met {
                return false;
            }
            if i == 1 {
                nums[i-1] = nums[i];
            } else if nums[i] < nums[i-2] {
                nums[i] = nums[i-1];
            } else {
                nums[i-1] = nums[i-2];
            }
            met = true;
        }
    }
    true
}
