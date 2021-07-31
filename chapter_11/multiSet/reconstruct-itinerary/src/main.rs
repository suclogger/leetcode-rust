fn main() {
    assert_eq!(find_itinerary(vec![vec![String::from("MUC"),String::from("LHR")],vec![String::from("JFK"),String::from("MUC")],
                                   vec![String::from("SFO"),String::from("SJC")], vec![String::from("LHR"),String::from("SFO")]]),
               vec![String::from("JFK"),String::from("MUC"),String::from("LHR"),String::from("SFO"),String::from("SJC")]);
    assert_eq!(find_itinerary(vec![vec![String::from("JFK"),String::from("SFO")],vec![String::from("JFK"),String::from("ATL")],vec![String::from("SFO"),String::from("ATL")],vec![String::from("ATL"),String::from("JFK")],vec![String::from("ATL"),String::from("SFO")]]),
               vec![String::from("JFK"),String::from("ATL"),String::from("JFK"),String::from("SFO"),String::from("ATL"),String::from("SFO")]
        );
}
/**
欧拉路径问题：Hierholzer算法
死胡同是要放到最后一个访问的
只有死胡同节点需要出栈
出栈顺序的倒序就是实际的遍历顺序

**/
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut hash_table = std::collections::HashMap::new();
    for ticket in tickets {
        hash_table.entry(ticket[0].to_string())
            .or_insert(Vec::new())
            .push(ticket[1].to_string());
    }
    for destination in hash_table.values_mut() {
        destination.sort_by(|a, b| a.cmp(&b));
    }

    let mut ans = Vec::new();
    let mut stack = Vec::new();
    stack.push(String::from("JFK"));

    while !stack.is_empty() {
        let top = stack[stack.len() - 1].to_string();
        if !hash_table.contains_key(&top) {
            stack.pop();
            ans.push(top.to_string());
        } else {
            let des = hash_table.get_mut(&top).unwrap();
            stack.push(des[0].to_string());
            des.remove(0);
            if des.is_empty() {
                hash_table.remove_entry(&top);
            }
        }
    }
    ans.into_iter().rev().collect()
}