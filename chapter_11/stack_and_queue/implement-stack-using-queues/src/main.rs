fn main() {
    println!("Hello, world!");
}

struct MyStack {
    from: Vec<i32>,
    to: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            from: Vec::new(),
            to: Vec::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        if self.to.is_empty() {
            self.from.push(x);
        } else {
            self.to.push(x);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.from.is_empty() && self.to.is_empty() {
            return 0;
        }
        let (from, to) = if self.from.is_empty() {(&mut self.to, &mut self.from)} else {(&mut self.from, &mut self.to)};
        while from.len() > 1 {
            to.push(from.remove(0));
        }
        from.remove(0)
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        return if self.from.is_empty() && self.to.is_empty() {
            return 0;
        } else if self.from.is_empty() {
            self.from[0]
        } else {
            self.to[0]
        }
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.from.is_empty() && self.to.is_empty()
    }
}
