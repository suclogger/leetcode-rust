fn main() {
    println!("Hello, world!");
}

pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut priority_queue = std::collections::BinaryHeap::new();
    for pile in piles {
        priority_queue.push(pile);
    }
    let mut k = k;
    while k > 0 && !priority_queue.is_empty() {
        let cur = priority_queue.pop().unwrap();
        priority_queue.push(cur - cur / 2);
        k -= 1;
    }

    let mut ans = 0;
    while !priority_queue.is_empty() {
        ans += priority_queue.pop().unwrap();
    }
    ans
}