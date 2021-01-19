fn main() {
    // 先把高的人放好即可
    let v = vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]];
    assert_eq!(vec![vec![5,0],vec![7,0],vec![5,2],vec![6,1],vec![4,4],vec![7,1]], reconstruct_queue(v));
}

pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    people.sort_unstable_by_key(|v| (-v[0], v[1]));
    let mut res = Vec::with_capacity(people.len());
    for person in people {
        res.insert(person[1] as usize, person);
    }
    res
}