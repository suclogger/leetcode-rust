fn main() {
    let v = vec![2,5,6,0,0,1,2];
    assert!(search(v, 0));
    let v = vec![2,5,6,0,0,1,2];
    assert!(!search(v, 3));
}

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }
    let mut l : i32 = 0;
    let mut r: i32 = nums.len() as i32;
    if nums[0] >= nums[nums.len() -1] {
        let mid : usize = (l + (r - l) /2) as usize;
        // Leetcode上需要通过Self::指向内部函数
        // return Self::search(nums[0..mid].to_vec(), target) || Self::search(nums[mid..nums.len()].to_vec(), target);
        return search(nums[0..mid].to_vec(), target) || search(nums[mid..nums.len()].to_vec(), target);
    }
    while l < r {
        let mid = l + (r - l) /2;
        if nums[mid as usize] == target {
            return true;
        } else if nums[mid as usize] > target {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    false
}