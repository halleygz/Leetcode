use std::cmp;
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut curr_a: i32 = capacity_a;
        let mut curr_b: i32 = capacity_b;
        
        let mut refills: i32 = 0;
        
        let mut left = 0 as usize;
        let mut right = plants.len()-1;
        
        
        while left <= right {
            
            if left == right {
                if cmp::max(curr_a, curr_b) < plants[left] {
                    refills += 1;
                }
                break;
            }
            
            if curr_a < plants[left] {
                refills += 1;
                curr_a = capacity_a;
            }
            curr_a -= plants[left];
            
            if curr_b < plants[right] {
                refills += 1;
                curr_b = capacity_b;
            }
            curr_b -= plants[right];
            
            left += 1;
            right -= 1;
        }
        
        return refills;
    }
}