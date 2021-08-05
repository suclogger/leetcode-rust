fn main() {
    assert_eq!(find_duplicate(vec![2,1,3,3]), 3);
}
/**
典型的一道 index_sort 题型

2  1  3  3
1  2  3  3

 **/
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut idx = 0;
    while idx < nums.len() {
        while nums[idx] as usize - 1 != idx {
            let val = nums[idx];
            if val == nums[val as usize - 1] {
                return nums[idx];
            }
            nums.swap(idx, val as usize - 1);
        }
        idx += 1;
    }
    0
}