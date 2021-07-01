use std::collections::VecDeque;

fn main() {
    // assert_eq!(vec![1], find_min_height_trees(4, vec![vec![1,0],vec![1,2],vec![1,3]]));
    assert_eq!(vec![0] as Vec<i32>, find_min_height_trees(1, vec![]));
}

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let n = n as usize;
    let mut ans: Vec<i32> = vec![];
    let mut d_set: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut degree: Vec<usize> = vec![0; n];
    for edge in edges.iter() {
        degree[edge[0] as usize] = degree[edge[0] as usize] + 1;
        degree[edge[1] as usize] = degree[edge[1] as usize] + 1;
        d_set[(edge[0] as usize)].push(edge[1]);
        d_set[(edge[1] as usize)].push(edge[0]);
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    for d in 0..n {
        if degree[d] == 1 {
            q.push_back(d);
        }
    }

    while !q.is_empty() {
        ans.clear();
        let s = q.len();
        for _ in 0..s {
            let node = q.pop_front().unwrap();
            ans.push(node as i32);
            for neighbor in d_set[node].iter() {
                degree[*neighbor as usize] = degree[*neighbor as usize] - 1;
                if degree[*neighbor as usize] == 1 {
                    q.push_back(*neighbor as usize);
                }
            }
        }
    }
    ans
}