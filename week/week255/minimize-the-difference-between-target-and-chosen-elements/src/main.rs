fn main() {
    assert_eq!(minimize_the_difference(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 13), 13);
}

pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut m_vec = vec![(0, 0); mat.len()];
    let mut min_sum = 0;
    let mut max_sum = 0;
    for (pos, v) in mat.iter().enumerate() {
        m_vec[pos].0 = *v.into_iter().min().unwrap();
        min_sum += m_vec[pos].0;
        m_vec[pos].1 = *v.into_iter().max().unwrap();
        max_sum += m_vec[pos].1;
    }
    if target <= min_sum {
        return min_sum - target;
    } else if target >= max_sum {
        return target - max_sum;
    } else {



    }



    1

}