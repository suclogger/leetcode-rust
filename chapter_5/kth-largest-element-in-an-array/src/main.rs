fn main() {
    let v = vec![3,2,1,5,6,4];
    assert_eq!(find_kth_largest(v, 2), 5);
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut nums = nums;
    quick_select(&mut nums, 0, len-1,  (len - k as usize) as usize)
}

fn quick_select(nums: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
    let partition = partition(nums, l, r);
    return if partition == k {
        nums[partition]
    } else if partition > 0 && partition > k {
        quick_select(nums, l, partition - 1, k)
    } else {
        quick_select(nums, partition + 1, r, k)
    }
}

fn partition(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let pivot = r;
    let mut partition_idx = l;
    for j in l..r {
        if nums[j] < nums[pivot] {
            swap(nums, partition_idx, j);
            partition_idx+=1;
        }
    }
    swap(nums, pivot, partition_idx);
    partition_idx
}

fn swap(nums : &mut Vec<i32>, l: usize, r: usize) {
    let t = nums[l];
    nums[l] = nums[r];
    nums[r] = t;
}