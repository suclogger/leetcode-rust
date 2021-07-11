fn main() {
    println!("Hello, world!");
}


pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![]
    }
    let mut ans: Vec<i32> = vec![0; nums.len() * 2];
    for i in 0..nums.len() {
        ans[i] = nums[i];
        ans[i + nums.len()] = nums[i];
    }
    ans
}