fn main() {
    let v = vec![
        vec![2,4],
        vec![3,4],
        vec![2,3],
        vec![1,2],
        vec![1,3],
        vec![1,4],
    ];
    assert_eq!(v, combine(4,2));
}

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut comb: Vec<i32> = vec![0; k as usize];
    backtracking(&mut ans, &mut comb, 1, 0, n as usize, k as usize);
    ans
}

fn backtracking(ans: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>, pos: usize, mut count: usize, n: usize, k: usize) {
    if count == k {
        ans.push(comb.to_vec());
        return;
    }
    for po in pos..=n {
        comb[count] = po as i32;
        count += 1;
        backtracking(ans, comb, po + 1, count, n, k);
        count -= 1;
    }
}