impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            for i in 0..(nums.len()-1) {
                nums[i] = (nums[i] + nums[i+1]) % 10;
            }
            _ = nums.pop();
        }
        nums.pop().unwrap()
    }
}