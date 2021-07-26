fn main() {
    assert_eq!(daily_temperatures(vec![73,74,75,71,69,72,76,73]), vec![1,1,4,2,1,1,0,0])
}
/**
单调栈的思路：

73,74,75,71,69,72,76,73

维护一个单调递增的栈，元素小于栈顶，或者栈为空才可入栈
73 -> 栈为空，入栈
74 -> 栈顶 73 小于 74 ，循环出栈,直到栈顶大于当前元素，
    在出栈时记录映射 73 -> 74，74入栈
75 -> 同74，75入栈
71 -> 栈顶 75 > 71 , 入栈
69 -> 栈顶 71 > 69，入栈
72 -> 栈顶 69 小于 72 ，循环出栈,直到栈顶大于当前元素
    在69出栈时记录映射 69 -> 72， 71->72 ，当前栈顶为75 > 72，停止出栈，72入栈
76 -> 栈顶 72 小于 76, 72出栈，记录72->76,76入栈
73 -> 入栈

输出映射关系，不存在映射关系的，输出0
因为要计算相对天数，所以单调栈直接保存下标，比较大小时直接从数组取值

**/

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; temperatures.len()];
    let mut monotonic_stack = Vec::new();
    for (idx, t) in temperatures.iter().enumerate() {
        if monotonic_stack.is_empty() || temperatures[monotonic_stack[monotonic_stack.len() - 1]] >= *t {
            monotonic_stack.push(idx);
        } else {
            while !monotonic_stack.is_empty() && temperatures[monotonic_stack[monotonic_stack.len() - 1]] < *t {
                let pre_day = monotonic_stack.pop().unwrap();
                ans[pre_day] = (idx - pre_day) as i32;
            }
            monotonic_stack.push(idx);
        }
    }
    ans
}