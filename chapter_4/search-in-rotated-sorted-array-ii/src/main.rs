fn main() {
    let v = vec![2,5,6,0,0,1,2];
    assert!(search(v, 0));
    let v = vec![2,5,6,0,0,1,2];
    assert!(!search(v, 3));
    let v = vec![1,0,1,1,1];
    assert!(search(v, 0));
    let v = vec![5,1,3];
    assert!(search(v, 3));
}

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() < 1 {
        return false;
    }
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l <= r {
        let mid = l + (r - l)/2;
        if nums[mid] == target {
            return true;
        }
        if nums[l] == nums[mid] {
            l+=1;
        } else if nums[mid] <= nums[r] {
            if target > nums[mid] && target <= nums[r] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        } else {
            if target < nums[mid] && target >= nums[l] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
    }
    false
}