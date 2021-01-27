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
    let mut r = nums.len();
    while l < r {
        // 要点1 向下取整的时候，左指针l是一定要动的
        let mid = l + (r - l)/2;
        if nums[mid] == target {
            return true;
        }
        if nums[l] == nums[mid] {
            l+=1;
        } else if nums[l] <= nums[mid] {
            // 两个元素时 l==mid，所有要考虑相等情况
            if target >= nums[l] && target < nums[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        } else {
            if target > nums[mid] && target <= nums[r-1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
    }
    false
}