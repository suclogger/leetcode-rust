fn main() {
    println!("Hello, world!");
}

pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    let mut ans = 0;
    for p in patterns  {
        if word.contains(&p) {
            ans +=1;
        }
    }
    ans
}