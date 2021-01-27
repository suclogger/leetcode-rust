fn main() {
    let v = vec![1,1,2,3,3,4,4,8,8];
    assert_eq!(2, single_non_duplicate(v));
    let v = vec![3,3,7,7,10,11,11];
    assert_eq!(10, single_non_duplicate(v));
    let v = vec![1,1,2];
    assert_eq!(2, single_non_duplicate(v));
    let v = vec![1,2,2,3,3];
    assert_eq!(1, single_non_duplicate(v));
}

pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let mid = l + (r - l)/2;
        let even = (r - mid) % 2 == 0;
        if mid > 0 && nums[mid] == nums[mid-1] {
            if even {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        } else if mid != r - 1 && nums[mid] == nums[mid + 1] {
            if even {
                r = mid ;
            } else {
                l = mid + 2;
            }
        } else {
            return nums[mid]
        }
    }
    nums[l]
}