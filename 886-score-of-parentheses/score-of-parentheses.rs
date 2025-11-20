impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for ch in s.chars() { 
            match ch  { 
                '(' => stack.push(0),
                _ => { 
                    let v = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += i32::max(2 * v, 1)
                }
            }
        }
        stack.pop().unwrap()
    }
}