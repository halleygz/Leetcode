impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut acc = x;
	let mut y = 0;
	while (acc > 0) {
		y = y * 10 + acc % 10;
		acc /= 10;
	}
	x == y
    }
}