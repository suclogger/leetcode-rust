fn main() {
    let v = vec![1,0,2];
    assert_eq!(candy(v), 5);
    let v = vec![1,3,4,5,2];
    assert_eq!(candy(v), 11);
    let v = vec![1,3,2,2,1];
    assert_eq!(candy(v), 7);
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut count_vec = vec![1];
    for  i in 1..ratings.len() {
        if ratings[i] > ratings[i-1] {
            count_vec.push(count_vec[i-1] + 1);
        } else {
            count_vec.push(1);
        }
    }
    for i in (0..ratings.len()-1).rev() {
        if ratings[i] > ratings[i+1] && count_vec[i] <= count_vec[i+1] {
            count_vec[i] = count_vec[i+1]+1;
        }
    }
    count_vec.iter().sum()
}