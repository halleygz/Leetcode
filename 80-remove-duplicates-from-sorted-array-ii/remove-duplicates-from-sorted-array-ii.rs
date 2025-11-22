impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
     *nums = nums
            .iter()
            .enumerate()
            .flat_map(|(i, &n)| {
                if i > 1 && nums[i - 2] == n {
                    None
                } else {
                    Some(n)
                }
            })
            .collect();
        nums.len() as i32   
    }
}