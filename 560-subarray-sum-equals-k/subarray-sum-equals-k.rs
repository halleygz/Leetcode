use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len: usize = nums.len();
      let mut hash: HashMap<i32, i32> = HashMap::new();
      let mut count: i32 = 0;
      let mut sum: i32 = 0;
      hash.insert(0, 1);
  
      for i in 0..len {
          sum += nums[i];
          if let Some(&val) = hash.get(&(sum - k)) {
              count += val;
          }
          *hash.entry(sum).or_insert(0) += 1;
      }
      count
    }
}