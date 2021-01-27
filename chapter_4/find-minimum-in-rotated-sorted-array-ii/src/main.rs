fn main() {
    let v = vec![1,3,5];
    assert_eq!(1, find_min(v));
    let v = vec![2,2,2,0,1];
    assert_eq!(0, find_min(v));
    let v = vec![1,3];
    assert_eq!(1, find_min(v));
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    find_min_inner(&nums, 0, nums.len())
}

fn find_min_inner(nums: &Vec<i32>, l: usize, r:usize) -> i32 {
    if l == r - 1 {
        return nums[l];
    }
    let mid = l + (r - l)/2;
    return if nums[mid] == nums[l] {
        find_min_inner(nums, l + 1, r)
    } else if nums[mid] >= nums[l] {
        // 左侧有序，比较左边界和右侧
        if mid + 1 >= r { return nums[l]; }
        let r_min = find_min_inner(nums, mid + 1, r);
        if r_min < nums[l] { r_min } else { nums[l] }
    } else {
        // 右侧有序，比较mid和左侧
        let l_min = find_min_inner(nums, l, mid);
        if l_min < nums[mid] { l_min } else { nums[mid] }
    }
}