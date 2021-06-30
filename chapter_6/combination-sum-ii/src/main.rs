fn main() {
    assert_eq!(combination_sum2(vec![10,1,2,7,6,1,5], 8), vec![
        vec![1, 7],
        vec![1, 2, 5],
        vec![2, 6],
        vec![1, 1, 6]
    ]);
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(candidates: &Vec<i32>, target: i32, idx: usize, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ans.push(cur.to_vec());
            return;
        }

        for cur_idx in idx..candidates.len() {
            if cur_idx > idx && candidates[cur_idx] == candidates[cur_idx - 1] {
                continue;
            }
            if candidates[cur_idx] > target {
                continue;
            }
            cur.push(candidates[cur_idx]);
            dfs(candidates, target - candidates[cur_idx], cur_idx + 1, cur, ans);
            cur.pop();
        }
    }

    let mut candidates = candidates;
    candidates.sort();
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut cur: Vec<i32> = vec![];
    dfs(&mut candidates, target, 0, &mut cur, &mut ans);
    ans
}