fn main() {
    assert_eq!(can_see_persons_count(vec![10,6,8,5,11,9]), vec![3,1,2,1,1,0]);
}
/**
一道没做出来的双周赛题：单调栈的思路
左边的人能看到右边的，意味着从右边往左边肯定是个单调递减的序列，如果是有非递增元素可以直接去除（因为反正也看不到）
[10,6,8,5,11,9]
栈为空，或者栈顶比当前元素小可以入栈
因为出栈的元素比当前元素和栈顶都要小，所以是要观测到的对象，栈顶元素也可以观测到，给这两部分计数
从9开始入栈，入栈
11来了，9出栈，11入栈，
5来了，入栈，
8来了，5出栈
6来了，入栈
10来了，依次出栈到11

**/
pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; heights.len()];
    let mut monotonic_stack = Vec::new();
    for (pos, h) in heights.iter().rev().enumerate() {
        let mut count = 0;
        while !monotonic_stack.is_empty() && monotonic_stack[monotonic_stack.len() - 1] <= h {
            monotonic_stack.pop();
            count = count + 1;
        }
        ans[heights.len() - 1 - pos] = if monotonic_stack.is_empty() {count} else {count + 1} as i32;
        monotonic_stack.push(h);
    }
    ans
}