impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack = Vec::new();

        for byte in s.bytes() {
            match byte{
                b')' => if stack.pop() != Some(b'(') {return false},
                b']' => if stack.pop() != Some(b'[') {return false},
                b'}' => if stack.pop() != Some(b'{') {return false},
                _ => stack.push(byte),
            }
        }

        stack.is_empty()
    }
}