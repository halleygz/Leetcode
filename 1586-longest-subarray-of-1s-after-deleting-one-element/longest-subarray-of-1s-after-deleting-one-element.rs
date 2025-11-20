impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let [mut last, mut cnt, mut ans] = [0; 3];
		let max = nums.len() as i32 - 1;
		for n in nums {
			if n == 1 {
				cnt += 1;
				ans = ans.max(cnt + last);
			} else {
				(last, cnt) = (cnt, 0);
			}
		}
		ans.min(max)
    }
}