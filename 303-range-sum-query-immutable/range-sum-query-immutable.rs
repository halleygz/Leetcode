struct NumArray {
    prefix: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut arr: Vec<i32> = nums;

        for i in 0..arr.len() - 1 {
            arr[i + 1] += arr[i]
        }

        return NumArray{prefix: arr}
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
         let (left, right) = (left as usize, right as usize);

        if left == 0 {
            return self.prefix[right]
        }

        return self.prefix[right] - self.prefix[left - 1]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */