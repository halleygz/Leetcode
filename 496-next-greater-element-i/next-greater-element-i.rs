impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater = vec![-1; nums2.len()];
        let mut stack = Vec::new();
        for (i, &num) in nums2.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if nums2[top] < num {
                    next_greater[top] = num;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        nums1
            .into_iter()
            .map(|num| next_greater[nums2.iter().position(|&n| n == num).unwrap()])
            .collect()
    }
}