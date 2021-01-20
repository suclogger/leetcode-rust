fn main() {
    let nums1 = &mut vec![1,2,3,0,0,0];
    // 3 2 1 256
    let m = 3;
    let nums2 = &mut vec![2,5,6];
    let n = 3;
    merge(nums1, m, nums2, n);
    assert_eq!(*nums1, vec![1,2,2,3,5,6]);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // 逆转，整理，写入，再逆转
    (*nums1).reverse();
    for i in 0..(m as usize) {
        (*nums1)[i] = (*nums1)[i+n as usize]
    }
    let mut l  = m  -1;
    let mut r  = 0;
    while l>=0 || r<n {
        let cur_idx =  m + n - (m - 1 - l + r) -1;
        if r<n  && (l <0 || (*nums1)[l as usize]> (*nums2)[r as usize]){
            (*nums1)[cur_idx as usize] = nums2[r as usize];
            r += 1;
        } else {
            (*nums1)[cur_idx as usize] = nums1[l as usize];
            l -= 1;
        }
    }
    (*nums1).reverse();
}