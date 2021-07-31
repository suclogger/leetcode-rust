fn main() {
    println!("Hello, world!");
}

/**
rust可以使用vec来模拟stack
用两个栈来模拟queue，可以在读或者写的时候翻转
**/

struct MyQueue {
    vec1: Vec<i32>,
    vec2: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {


    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {vec1:Vec::new(), vec2:Vec::new()}
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.vec1.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.vec2.is_empty() {
            while !self.vec1.is_empty() {
                self.vec2.push(self.vec1.pop().unwrap());
            }
        }
        return self.vec2.pop().unwrap();
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.vec2.is_empty() {
            while !self.vec1.is_empty() {
                self.vec2.push(self.vec1.pop().unwrap());
            }
        }
        return self.vec2[self.vec2.len() - 1];
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.vec1.is_empty() && self.vec2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */