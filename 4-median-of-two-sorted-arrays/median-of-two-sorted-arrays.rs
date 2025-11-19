impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
       let mut nums: Vec<&i32> = nums1.iter().chain(nums2.iter()).collect();
        nums.sort();

        if nums.is_empty(){
            return 0f64;
        }

        if nums.len() == 1{
            return *nums[0] as f64;
        }

        if nums.len() % 2 == 0{
            (*nums[(nums.len()/2)-1] as f64 + *nums[(nums.len()/2)]  as f64)/2f64
        }else{
            *nums[nums.len()/2]  as f64
        }
    }
}