fn main() {
    assert_eq!(add_rungs(vec![1,3,5,10], 2), 2);
    assert_eq!(add_rungs(vec![3], 1), 2);
}

pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {

    let mut new_steps = 0;
    for i in 0..rungs.len() {
        let diff =  if i == 0 {
            rungs[0]
        } else {
            rungs[i] - rungs[i-1]
        };
        new_steps = new_steps + (if diff > dist { if diff % dist == 0 {diff/dist - 1} else {diff/dist} } else {0});
    }
    new_steps
}