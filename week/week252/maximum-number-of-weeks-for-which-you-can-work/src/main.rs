fn main() {
    assert_eq!(number_of_weeks(vec![1,2,3]), 6);
    assert_eq!(number_of_weeks(vec![5,2,1]), 7);
}


pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    // heap for current max work
    let mut heap = std::collections::BinaryHeap::new();
    for ms in milestones {
        heap.push(ms);
    }
    let mut pre = 0;
    while !heap.is_empty() {
        let cur = heap.pop().unwrap();
        if pre > 0 {
            heap.push(pre);
        }
        ans = ans + 1;
        pre = cur - 1;
    }
    ans
}