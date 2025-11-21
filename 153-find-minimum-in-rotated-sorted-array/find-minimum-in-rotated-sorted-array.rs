impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut arr_begin = 0;

        let mut begin = 0;
        let mut end = nums.len();
        while begin < end
        {
            let mid = (end + begin) / 2;

            if nums[mid] >= nums[0]
            {
                begin = mid + 1;
            }
            else
            {
                end = mid;
                arr_begin = mid;
            }
        }        

        return nums[arr_begin];
    }
}