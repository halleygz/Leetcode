impl Solution {
    pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0u16; 101];
        for &num in &nums {
            counts[num as usize] += 1;
        }

        let mut total = 0;
        for c in &mut counts[..] {
            let sum = total + *c;
            *c = total;
            total = sum;
        }
        
        for num in &mut nums {
            *num = counts[*num as usize] as i32;
        }
        
        nums
    }
}