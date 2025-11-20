impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;

        fn reverse(slice: &mut [i32]) {
            let mut i = 0;
            let mut j = slice.len().saturating_sub(1);
            while i < j {
                slice.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        reverse(nums);
        reverse(&mut nums[..k]);
        reverse(&mut nums[k..]);
    }
}