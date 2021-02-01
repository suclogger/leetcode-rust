fn main() {
    // assert_eq!(vec![1,2], top_k_frequent(vec![1,1,1,2,2,3], 2));
    // assert_eq!(vec![1,2], top_k_frequent(vec![1,1,1,2,2,33333333], 2));
    assert_eq!(vec![1,2], top_k_frequent(vec![1,1,1,2,2,3], 2));
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for num in nums {
        let v = map.entry(num).or_insert(0);
        *v += 1;
    }
    let mut v: Vec<_> = map.into_iter().collect();
    let len = v.len();
    quick_select(&mut v, 0, len - 1, (len - k as usize) as usize);
    v[(len - k as usize)..(v.len() as usize)].into_iter().map(|x|x.0).collect()
}

fn quick_select(nums: &mut Vec<(i32, i32)>, l: usize, r: usize, k: usize) {
    let partition_idx = partition(nums, l, r);
    if partition_idx == k {
        return;
    } else if partition_idx < k {
        quick_select(nums, partition_idx + 1, r, k);
    } else if partition_idx > 0 {
        quick_select(nums, l, partition_idx - 1, k);
    }
}

fn partition(nums: &mut Vec<(i32, i32)>, l: usize, r: usize) -> usize {
    let pivot_element = nums[r].1;
    let mut partition_idx = l;
    for j in l..r {
        if nums[j].1 < pivot_element {
            swap(nums, partition_idx, j);
            partition_idx+=1;
        }
    }
    swap(nums, partition_idx, r);
    partition_idx
}

fn swap(nums: &mut Vec<(i32, i32)>, l: usize, r: usize) {
    let tmp = nums[l];
    nums[l] = nums[r];
    nums[r] = tmp;
}