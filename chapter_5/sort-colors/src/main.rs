fn main() {
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut idx = 0;
    while idx <= r && r > 0 {
        if nums[idx] == 2 {
            Self::swap(nums, idx, r);
            if r > 0 {
                r-=1;
            }
        } else if nums[idx] == 0 {
            Self::swap(nums, idx, l);
            l+=1;
            idx+=1;
        } else {
            idx+=1;
        }
    }
}

fn swap(nums: &mut Vec<i32>, l: usize, r: usize) {
    let tmp = nums[l];
    nums[l] = nums[r];
    nums[r] = tmp;
}