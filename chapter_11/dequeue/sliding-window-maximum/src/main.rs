use std::collections::VecDeque;

fn main() {
    assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    assert_eq!(max_sliding_window(vec![1,-1], 1), vec![1,-1]);
}
/**

双端队列的经典题型
维护一个单调递减的双端队列
1. 每来一个元素，沿队尾往头部维护单调递减顺序
2. 并确保队头元素未过期
3. 满足k个后每步输出队头
示例：
    [1,3,-1,-3,5,3,6,7], k = 3
检查1 队为空，入队尾
检查3 发现当前队尾1小，出队尾，队为空，入队尾
检查-1 满足递减，，入队尾，输出队头3
检查-3 满足递减，，入队尾，输出队头3
检查5 发现当前队尾-3，-1，3均小，出队尾，队为空，入队尾，输出队头5
检查3 满足递减，入队尾，输出队头5
检查6 发现当前队尾3，5小，出队尾，输出队头6
检查7 发现当前队尾6小，出队尾，输出队头7
最终得到：
    [3,3,5,5,6,7]
**/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut deque = VecDeque::new();
    for i in 0..nums.len() {
        while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
            deque.pop_back();
        }
        deque.push_back(i);
        while !deque.is_empty() && i >= k as usize && *deque.front().unwrap() <= i - k as usize {
            deque.pop_front();
        }

        if i as i32 > k - 2 {
            ans.push(nums[*deque.front().unwrap()] as i32);
        }
    }
    ans
}