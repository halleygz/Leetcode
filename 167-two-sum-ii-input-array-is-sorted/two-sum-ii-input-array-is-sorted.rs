impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut start:usize = 0; 
       let mut end = numbers.len() -1;
    
        while start < end {
            if numbers[start] + numbers[end] == target{
                return vec![(start + 1)  as i32, (end + 1)as i32];
            } else if numbers[start] + numbers[end] < target{
                start += 1;
            } else if numbers[start] + numbers[end] > target{
                end -= 1;
            } else {
                start +=1;
                end +=1;
            }
        }
        vec![]
    }
}