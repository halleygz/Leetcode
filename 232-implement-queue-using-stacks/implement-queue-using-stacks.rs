#[derive(Default)]
struct MyQueue {
    forward_stack: Vec<i32>,
    backward_stack: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Default::default()
    }
    
    fn push(&mut self, x: i32) {
        self.backward_stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.shift_forwards();
        self.forward_stack.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self.shift_forwards();
        *self.forward_stack.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.forward_stack.is_empty() && self.backward_stack.is_empty()
    }

    fn shift_forwards(&mut self) {
        if self.forward_stack.is_empty() {
            loop {
                match self.backward_stack.pop() {
                    Some(value) => self.forward_stack.push(value),
                    None => break
                }
            }
        }
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