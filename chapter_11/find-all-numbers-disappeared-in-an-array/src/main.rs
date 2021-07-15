fn main() {
    assert_eq!(find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
}


pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut ans = vec![];
    for i in 0..nums.len() {
        while i + 1 != nums[i] as usize && nums[nums[i] as usize - 1] != nums[i] {
            let nums_i = nums[i] as usize;
            nums.swap(i,  nums_i - 1);
        }
    }

    for i in 0..nums.len() {
        if i + 1 != nums[i] as usize{
            ans.push(i as i32 + 1);
        }
    }
    ans
}