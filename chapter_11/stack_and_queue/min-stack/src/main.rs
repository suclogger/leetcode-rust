fn main() {
    println!("Hello, world!");
}

/**
考虑到栈的特性，比当前已知的最小的值还要小的值，都需要记录，但是大的值都可以丢弃，因为大的值后来肯定先出，永远不会成为当前的最小值
所以可以用两个栈来记录

**/

struct MinStack {
    vec: Vec<i32>,
    vec_s: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { vec: Vec::new(), vec_s: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if self.vec_s.is_empty() || self.vec_s[self.vec_s.len() - 1] >= val {
            self.vec_s.push(val);
        }
        self.vec.push(val);
    }

    fn pop(&mut self) {
        if self.vec.is_empty() {
            return;
        }
        let val = self.vec.pop().unwrap();
        if !self.vec_s.is_empty() && self.vec_s[self.vec_s.len() - 1] == val {
            self.vec_s.pop();
        }
    }

    fn top(&self) -> i32 {
        self.vec[self.vec.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.vec_s[self.vec_s.len() - 1]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */