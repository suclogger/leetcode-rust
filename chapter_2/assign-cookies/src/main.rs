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
    let mut cookieIdx = 0;
    let mut satisfiedKids = 0;
    for kid in g {
        for x in cookieIdx..s.len() {
            cookieIdx=x+1;
            if s[x] >= kid {
                satisfiedKids+=1;
                break;
            }
        }
        
    }
    satisfiedKids
}
