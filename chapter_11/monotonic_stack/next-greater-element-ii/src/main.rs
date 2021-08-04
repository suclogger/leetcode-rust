fn main() {
    assert_eq!(next_greater_elements(vec![1,2,1]), vec![2,-1,2]);
}
/**
还是维护一个单调栈的思路不变，因为是环形，所以需要额外对-1的节点进行第二次处理，等于整体遍历两遍

**/

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut monotonic_stack = Vec::new();
    let mut ans = vec![-1; nums.len()];
    for (pos, num) in nums.iter().rev().enumerate() {
        while !monotonic_stack.is_empty() && nums[monotonic_stack[monotonic_stack.len() - 1]] <=  *num {
            monotonic_stack.pop();
        }
        if !monotonic_stack.is_empty() {
            ans[nums.len() - 1 - pos] = monotonic_stack[monotonic_stack.len() - 1] as i32;
        }
        monotonic_stack.push(nums.len() - 1 - pos);
    }

    for (pos, num) in ans.iter_mut().rev().enumerate() {
        if *num != -1 {
            continue;
        }
        while !monotonic_stack.is_empty() && nums[monotonic_stack[monotonic_stack.len() - 1]] <=  nums[nums.len() - 1 - pos]{
            monotonic_stack.pop();
        }
        if !monotonic_stack.is_empty() {
            *num = monotonic_stack[monotonic_stack.len() - 1]  as i32;
        }
    }
    ans.iter().map(|x| if *x != -1 {nums[*x as usize]} else {-1}).collect()
}