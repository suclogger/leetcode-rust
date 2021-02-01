fn main() {
    // assert_eq!("eetr", frequency_sort(String::from("tree")));
    assert_eq!("cccaaa", frequency_sort(String::from("cccaaa")));
    assert_eq!("Aabb", frequency_sort(String::from("bbAa")));
}

pub fn frequency_sort(s: String) -> String {
    if s.len() < 1 {
        return s
    }
    let mut map = std::collections::HashMap::new();
    for entry in s.char_indices() {
        let element = map.entry(entry.1).or_insert(0);
        *element += 1;
    }
    let len = map.len();
    let mut nums = map.into_iter().collect();
    quick_sort(&mut nums, 0, len-1);
    nums.reverse();
    let mut string = String::new();
    for c in nums {
        for _ in 0..c.1 {
            string.push(c.0);
        }
    }
    string
}

fn quick_sort(nums: &mut Vec<(char, i32)>, l: usize, r: usize) {
    if l < r {
        let p = partition(nums, l, r);
        if p > 0 {
            quick_sort(nums, l, p - 1);
        }
        quick_sort(nums, p + 1, r);
    }
}

fn partition(nums: &mut Vec<(char, i32)>, l: usize, r: usize) -> usize {
    let pivot_element = nums[r].1;
    let mut partition_idx = l;
    for j in l..r {
        if nums[j].1 < pivot_element {
            swap(nums, partition_idx, j );
            partition_idx += 1;
        }
    }
    swap(nums, partition_idx, r );
    partition_idx
}

fn swap(nums: &mut Vec<(char, i32)>, l: usize, r: usize) {
    let tmp = nums[l];
    nums[l] = nums[r];
    nums[r] = tmp;
}