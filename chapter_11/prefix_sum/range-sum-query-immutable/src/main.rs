fn main() {
    println!("Hello, world!");
}

/**
前缀和的入门题
维护前缀和数组即可

 **/

struct NumArray {
    prefix_sum: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut psum = vec![0; nums.len()];
        for (pos, num) in nums.into_iter().enumerate() {
            psum[pos] = if pos > 0 { psum[pos - 1] + num } else { num };
        }
        NumArray {
            prefix_sum: psum
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left < 0 || right < 0 || left > self.prefix_sum.len() as i32 - 1
            || right > self.prefix_sum.len() as i32 - 1 || left > right {
            return 0;
        }
        self.prefix_sum[right as usize] - if left > 0 { self.prefix_sum[left as usize - 1] } else { 0 }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */