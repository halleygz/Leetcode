impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let iter = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        iter.clone().eq(iter.rev())
    }
}