fn main() {
    let mut array = Vec::new();
    array.push(vec![1,2,3,4]);
    array.push(vec![5,6,7,8]);
    array.push(vec![9,10,11,12]);
    array.push(vec![13,14,15,16]);

    let ans = bfs(array.to_vec());

    for n in ans {
        println!("{}", n);
    }
}

fn bfs(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::new();
    if nums.len() == 0 {
        return ans;
    }
    let mut queue = Vec::new();
    let mut set = std::collections::HashSet::new();
    queue.push((0, nums[0].len() -1));
    while !queue.is_empty() {
        let (i, j) = queue.remove(0);
        ans.push(nums[i][j]);
        if j  >= 1 {
            if !set.contains(&(i, j - 1)) {
                queue.push((i,j - 1));
                set.insert((i,j - 1));
            }

        }
        if i < nums.len()-1 {
            if !set.contains(&(i + 1, j)) {
                queue.push((i+1,j));
                set.insert((i+1,j));
            }
        }

    }
    ans
}

