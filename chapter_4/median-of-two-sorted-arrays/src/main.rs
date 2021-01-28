fn main() {
    let v1 = vec![1,3];
    let v2 = vec![2];
    assert_eq!(2.0, find_median_sorted_arrays(v1, v2));
    let v1 = vec![2];
    let v2 = vec![1,3,4];
    assert_eq!(2.5, find_median_sorted_arrays(v1, v2));

}
/**
中位数:
    如果m+n是奇数 => (m + n) / 2
    如果m+n是偶数 => (m + n) / 2 和 (m + n) / 2 - 1 的平均值
**/

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    return if (m + n) % 2 == 1 {
        let mid = (m + n) / 2;
        find_median_sorted_arrays_inner(&nums1, &nums2, mid + 1) as f64
    } else {
        let mid1 = (m + n) / 2 - 1;
        let mid2 = (m + n) / 2;
        (find_median_sorted_arrays_inner(&nums1, &nums2, mid1 + 1) +
            find_median_sorted_arrays_inner(&nums1, &nums2, mid2 + 1)) as f64  / 2.0
    }
}

fn find_median_sorted_arrays_inner(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k : usize) -> i32 {
    let mut l = 0;
    let mut r = 0;
    loop {
        if l == nums1.len() || r == nums2.len() {
            break if l == nums1.len() { nums2[k - 1 + r] } else { nums1[k - 1 + l] }
        }
        if k == 1 {
            break if nums1[l] > nums2[r] { nums2[r] } else { nums1[l] }
        }

        let new_l = std::cmp::min(nums1.len(), l + k/2) - 1;
        let new_r = std::cmp::min(nums2.len(), r + k/2) - 1;
        if nums1[new_l] <= nums2[new_r] {
            k -= new_l - l + 1;
            l = new_l + 1;
        } else {
            k -= new_r - r + 1;
            r = new_r + 1;
        }
    }
}