fn main() {
    find_different_binary_string(vec![String::from("00"), String::from("01")]);
}

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let mut array = vec![0; 2_i32.pow(nums.len() as u32) as usize];
    let n = nums.len();
    for num in nums {
        let m = u32::from_str_radix(&num, 2).unwrap() as usize;
        array[m] = 1;
    }
    for (pos, val) in array.iter().enumerate() {
        if *val == 0 {
            let ori = format!("{:b}", pos);
            if ori.len() < n {
                let mut s = String::new();
                for _ in 0..(n - ori.len()) {
                    s.push_str(&String::from("0"));
                }
                s.push_str(&ori);
                return s;
            } else {
                return ori;
            }
        }
    }
    String::new()
}