impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut modulo = 0;

        for bit in nums {
            modulo = ((modulo << 1) + bit) % 5;
            result.push(modulo == 0);
        }
        result
    }
}