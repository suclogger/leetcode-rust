fn main() {
    let numbers = vec![2, 7, 11, 15];
    assert_eq!(two_sum(numbers, 9), vec![1,2]);
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len()-1;
    while l < r {
        if numbers[l] + numbers[r] == target {
            break;
        }
        if numbers[l] + numbers[r] > target {
            r -= 1;
        } else {
            l += 1;
        }
    }
    if l == r { vec![] } else { vec![(l + 1) as i32,( r + 1) as i32] }
}