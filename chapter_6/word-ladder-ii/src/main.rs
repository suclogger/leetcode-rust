use std::collections::{HashMap, HashSet};
fn main() {
    assert_eq!(find_ladders( String::from("hit"), String::from("cog"), vec![String::from("hot"),String::from("dot"),String::from("dog"),String::from("lot"),String::from("log"),String::from("cog")]),
               vec![
                   vec!["hit","hot","dot","dog","cog"],
                   vec!["hit","hot","lot","log","cog"]
               ]);
}

pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = vec![];
    if !word_list.contains(&end_word) {
        return res.to_vec();
    }

    let mut reverse =  false;
    let mut found =  false;
    let mut q1 = vec![begin_word.to_string()];
    let mut q2 = vec![end_word.to_string()];
    let mut next: HashMap<String, Vec<String>> = HashMap::new();
    let mut dict: HashSet<String> = HashSet::new();
    for word in word_list {
        if !word.eq(&begin_word) && !word.eq(&end_word) {
            dict.insert(word.to_string());
        }
    }

    while !q1.is_empty() && !q2.is_empty() && !found {

        if q1.len() > q2.len() {
            reverse = !reverse;
            let tmp = q1;
            q1 = q2;
            q2 = tmp;
        }

        for word in q1.iter() {
            dict.remove(word);
        }
        for word in q2.iter() {
            dict.remove(word);
        }

        let mut q: Vec<String> = vec![];
        for cur_str in q1.iter() {
            let l = cur_str.len();
            let mut s_char_array: Vec<char> = cur_str.chars().collect();
            for i in 0..l {
                let cur_char = s_char_array[i];
                for c in b'a'..=b'z' {
                    s_char_array[i] = c as char;
                    let new_str: String = s_char_array.iter().collect();
                    if q2.contains(&new_str) {
                        found = true;
                        let mut parent = cur_str.to_string();
                        let mut child = new_str;
                        if reverse {
                            parent = child;
                            child = cur_str.to_string();
                        }
                        let mut next_set = next.entry(parent).or_insert(vec![]);
                        next_set.push(child);
                    } else if dict.contains(&new_str) && !found {
                        let mut parent = cur_str.to_string();
                        let mut child = new_str.to_string();
                        if reverse {
                            parent = child;
                            child = cur_str.to_string();
                        }
                        let mut next_set = next.entry(parent).or_insert(vec![]);
                        next_set.push(child);
                        q.push(new_str);
                    }


                }
                s_char_array[i] = cur_char;
            }
        }
        // swap q & q1
        let tmp = q1;
        q1 = q;
        q = tmp;
    }


    if found {
        dfs(&next, end_word, &begin_word, &mut vec![begin_word.to_string()], &mut res);
    }
    res.to_vec()
}


fn dfs(next: &HashMap<String, Vec<String>>, end_word: String, cur_word:&str,
       path:&mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if end_word.eq(cur_word) {
        res.push(path.to_vec());
    }
    match next.get(cur_word) {
        Some(list) => {
            for word in list {
                path.push(word.to_string());
                dfs(next, end_word.to_string(), word, path, res);
                path.pop();
            }
        },
        None => { }
    }
}

