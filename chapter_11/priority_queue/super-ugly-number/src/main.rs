use std::cmp::Reverse;

fn main() {
    // assert_eq!(nth_super_ugly_number(12, vec![2,7,13,19]), 32);
    assert_eq!(nth_super_ugly_number(100000, vec![7,19,29,37,41,47,53,59,61,79,83,89,101,103,109,127,131,137,139,157,167,179,181,199,211,229,233,239,241,251]), 1092889481);
    //100000
    // [7,19,29,37,41,47,53,59,61,79,83,89,101,103,109,127,131,137,139,157,167,179,181,199,211,229,233,239,241,251]


}
/**
1,2,7,13,19



**/
pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut priority_queue = std::collections::BinaryHeap::new();
    priority_queue.push(Reverse(1));
    let mut max = 1;
    let mut idx = 1;
    while !priority_queue.is_empty() && idx <= n {
        let Reverse(cur) = priority_queue.pop().unwrap();
        while !priority_queue.is_empty() && priority_queue.peek().unwrap().0 == cur {
            priority_queue.pop();
        }
        max = std::cmp::max(max, cur);
        for prime in &primes {
            priority_queue.push(Reverse(cur as i64 * *prime as i64));
        }
        idx += 1;
    }
    max as i32
}