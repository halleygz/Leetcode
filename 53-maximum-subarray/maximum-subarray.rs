impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cur, mut max) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            cur = match cur < 0 {
                true => nums[i],
                _ => cur + nums[i]
            };
            max = max.max(cur);
        }
        max
    }
}