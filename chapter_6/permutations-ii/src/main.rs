use std::collections::HashSet;

fn main() {
    assert_eq!(vec![vec![1,1,2],
                    vec![1,2,1],
                    vec![2,1,1]], permute_unique(vec![1,1,2]));
    // assert_eq!(permute_unique(vec![2,2,1,1]), vec![vec![1,1,2,2],vec![1,2,1,2],vec![1,2,2,1],vec![2,1,1,2],vec![2,1,2,1],vec![2,2,1,1]]);
    assert_eq!(permute_unique(vec![0, 3, 3, 3]
    ), vec![vec![0,3,3,3],vec![3,0,3,3],vec![3,3,0,3],vec![3,3,3,0]]);
}

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    fn back_track(nums: &mut Vec<i32>, level: usize, ans: &mut Vec<Vec<i32>>) {
        if level >= nums.len() {
            ans.push(nums.to_vec());
        }
        let mut seen: HashSet<i32> = HashSet::new();
        for idx in level..nums.len() {
            if seen.contains(&nums[idx])  {
                continue;
            }
            seen.insert(nums[idx]);
            nums.swap(level, idx);
            back_track(nums, level + 1, ans);
            nums.swap(level, idx);
        }
    }
    let mut nums = nums;
    back_track(&mut nums, 0, &mut ans);
    ans
}