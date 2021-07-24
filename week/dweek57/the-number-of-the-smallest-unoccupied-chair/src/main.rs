fn main() {
    // assert_eq!(smallest_chair(vec![vec![1,4],vec![2,3],vec![4,6]], 1), 1);
    assert_eq!(smallest_chair(vec![vec![3,10],vec![1,5],vec![2,6]], 0), 0);
}


pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    // 通过到达事件映射用户
    let mut usr_map = std::collections::HashMap::new();
    for (pos, time) in times.iter().enumerate() {
        usr_map.entry(time[0]).or_insert(pos);
    }

    let mut seats  = vec![0; times.len()];
    let mut times = times;
    times.sort_by(|a, b| a[0].cmp(&b[0]));
    for time in times {
        for (pos, s) in seats.to_vec().iter().enumerate() {
            if *s <= time[0] {
                seats[pos] = time[1];
                match usr_map.get(&time[0]) {
                    Some(usr) => if *usr == target_friend as usize {
                        return pos as i32;
                    },
                    None => println!("s unreviewed.")
                }
                break;
            }
        }
    }
    0
}