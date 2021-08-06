fn main() {
    // println!("{}", binary_search(&vec![1,2,4,5], 1));

}
/**
先尝试了用田忌赛马的思路逐个比对，发现会一直TLE, 时间复杂度是 O(n*logn)
pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    fn binary_search(nums: &Vec<i32>, num: i32) -> usize{
        let mut a = 0;
        let mut b = nums.len() - 1;
        while a < b {
            let m = a + (b - a) / 2;
            if nums[m] < num {
                a = m + 1;
            } else {
                b = m;
            }
        }
        a
    }
    let mut nums1 = nums1;
    nums1.sort();
    let mut ans = Vec::new();
    for num in nums2 {
        let mut found = false;
        let pos = binary_search(&nums1, num);
        if pos == nums1.len() - 1 && nums1[pos] <= num {
            ans.push(nums1[0]);
            nums1.remove(0);
        } else {
            ans.push(nums1[pos]);
            nums1.remove(pos);
        }
    }
    ans
}


参考官方答案后，对两个数组直接排序，时间复杂度是 O(2n) = O(n)
 **/
pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1_sorted = nums1.to_vec();
    let mut nums2_sorted = nums2.to_vec();
    nums1_sorted.sort();
    nums2_sorted.sort();

    let mut remain = Vec::new();
    let mut hash_table = std::collections::HashMap::new();

    let mut i = 0;
    let mut j = 0;

    while i < nums1_sorted.len() {
        if nums1_sorted[i] > nums2_sorted[j] {
            hash_table.entry(nums2_sorted[j]).or_insert(Vec::new()).push(nums1_sorted[i]);
            j+=1;
        } else {
            remain.push(nums1_sorted[i]);
        }
        i+=1;
    }

    let mut ans = Vec::new();
    for num in nums2 {
        if let Some(val) = hash_table.get_mut(&num) {
            ans.push(val.remove(0));
            if val.is_empty() {
                hash_table.remove(&num);
            }
        } else {
            ans.push(remain.pop().unwrap())
        }
    }
    ans
}