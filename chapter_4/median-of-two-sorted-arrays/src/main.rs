fn main() {
    let v1 = vec![1,3];
    let v2 = vec![2];
    assert_eq!(2.0, find_median_sorted_arrays(v1, v2));

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
        find_median_sorted_arrays_inner(&nums1, &nums2, mid + 1) * 1.0
    } else {
        let mid1 = (m + n) / 2 - 1;
        let mid2 = (m + n) / 2;
        (find_median_sorted_arrays_inner(&nums1, &nums2, mid1 + 1) +
            find_median_sorted_arrays_inner(&nums1, &nums2, mid2 + 1)) / 2.0
    }
}

fn find_median_sorted_arrays_inner(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k : usize) -> i32 {
    // 特殊处理
    if nums1.len() + nums2.len() == 0 {
        return 0;
    }
    if nums1.len() + nums2.len() == 1 {
        return if nums1.len()==1 { nums1[0] } else { nums2[0] }
    }

    let mut l = 0;
    let mut r = 0;
    while k > 0 {
        // nums1 的第 k/2 -1 与 nums2 的第 k/2 -1比较，先考虑下边界
        if k/2 - 1 >= nums1.len() {
            if nums1.len() == 0 {
                return nums2[k-1];
            }
            let l_max = nums1[nums1.len() - 1];
            let r_k = nums2[k/2 -1];
            if l_max < r_k {
                // 实际剔除的量
                k -= nums1.len() - l;
                l = nums1.len() - 1;
            }
        }

        if nums1[k/2-1 + l] <= nums2[k/2-1 + r] {
            l = k/2;
        } else {
            r = k/2;
        }
        k -= (k/2 - 1);
    }
    if l < nums1.len() - 1 { nums1[l] } else { nums2[r] }
}