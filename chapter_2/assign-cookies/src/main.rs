fn main() {
    let g = vec![1,2,3];
    let s = vec![1,1];
    println!("answer:{}", find_content_children(g, s))
    
}

fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    g.sort();
    s.sort();
    let mut gi = 0;
    let mut si = 0;
    while gi < g.len() && si < s.len() {
        if g[gi] <= s[si] {
            gi+=1;
        }
        si+=1;
    }
    gi as i32
}
