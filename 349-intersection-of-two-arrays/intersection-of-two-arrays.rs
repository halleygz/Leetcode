impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersection = Vec::new();
        for i in 0..nums1.len() {
            if nums2.contains(&nums1[i]) && !intersection.contains(&nums1[i]) {
                intersection.push(nums1[i]);
            }
        }
        intersection
    }
}