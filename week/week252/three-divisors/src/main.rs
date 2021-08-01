fn main() {
    println!("Hello, world!");
}

pub fn is_three(n: i32) -> bool {
    pub fn is_three(n: i32) -> bool {
        if n < 4 {
            return false;
        }
        let mut found = false;
        for i in 2..=n/2 {
            if n % i == 0 && n /i != i  {
                return false;
            } else if n % i == 0 && n /i == i {
                found = true;
            }
        }
        return found;
    }
}