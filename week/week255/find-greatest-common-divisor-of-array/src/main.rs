fn main() {
    println!("Hello, world!");
}

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for num in nums {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }
    for i in (1..=min).rev().into_iter() {
        if max % i == 0 && min % i == 0 {
            return i;
        }
    }
    0
}