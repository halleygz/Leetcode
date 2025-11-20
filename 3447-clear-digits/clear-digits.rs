impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = String::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                result.pop();
            } else {
                result.push(ch);
            }
        }

        result
    }
}