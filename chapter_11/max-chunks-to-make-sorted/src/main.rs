fn main() {
    assert_eq!(max_chunks_to_sorted(vec![4,3,2,1,0]), 1);
    assert_eq!(max_chunks_to_sorted(vec![1,0,2,3,4]), 4);
}
/**
思路：可以往前缀和上思考
比如：
[2  1]  3  3  4
[1  2]  3  3  4 -- 有序结果数组
那原数组与有序数组前缀和相等的连续元素，里面的元素肯定跟有序数组的元素是一样的（虽然顺序不一样）#同分异构#

**/

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut ans = 0;
    let mut arr_sorted = arr.to_vec();
    arr_sorted.sort();
    for i in 0..arr.len() {
        sum1 = sum1 + arr[i];
        sum2 = sum2 + arr_sorted[i];
        if sum1 == sum2 {
            ans = ans + 1;
            sum1 = 0;
            sum2 = 0;
        }
    }
    ans
}