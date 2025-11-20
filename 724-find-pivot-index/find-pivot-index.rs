impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;

        for (i, v) in nums.iter().enumerate() {
            if left_sum * 2 + v == sum {
                return i as i32;
            }
            left_sum += v;
        }
        return -1;
    }
}