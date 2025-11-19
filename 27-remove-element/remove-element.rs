impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count: i32 = 0;
    let mut emp_vec: Vec<i32> = vec![];
    for idx in 0..nums.len() {
        if nums[idx] == val {
            count += 1;
            continue;
        } 
        emp_vec.push(nums[idx]);
    }
    let result: i32 = emp_vec.len() as i32;
    nums.clear();
    nums.extend(emp_vec);
    result
    
    
    }
}