use std::collections::HashMap;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
       if nums.len() == 0 {
            return 0;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max = 1;
        for i in nums {
            if map.get(&i).is_some() {
                continue;
            }
            map.insert(i, 1);
            if let Some(&count) = map.get(&(i - 1)) {
                map.insert(i, count + 1);
                map.insert(i - count, count + 1);
                max = max.max(count + 1);
            }
            if let Some(&count) = map.get(&(i + 1)) {
                let mered_count = count + map.get(&i).unwrap();
                map.insert(i + 1 - map.get(&i).unwrap(), mered_count);
                map.insert(i + map.get(&(i + 1)).unwrap(), mered_count);
                max = max.max(mered_count);
            }
        }
        max 
    }
}