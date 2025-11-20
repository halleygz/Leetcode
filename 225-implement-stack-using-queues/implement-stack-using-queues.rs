struct MyStack {
    data:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack{
            data: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        match self.data.pop(){
            Some(x) => return x,
            None => 0
        }
    }
    
    fn top(&self) -> i32 {
        let last = self.data.len()-1;
        self.data[last]
    }
    
    fn empty(&self) -> bool {
        self.data.len() == 0
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */