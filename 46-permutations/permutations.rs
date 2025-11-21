impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut p: Vec<i32> = vec![];

        Self::_permute(nums, &mut res, &mut p);
        res
    }

    fn _permute(nums: Vec<i32>, res: &mut Vec<Vec<i32>>, p: &mut Vec<i32>) {
        if nums.is_empty() {
            res.push(p.to_vec());
            return;
        }
        for (i, &value) in nums.iter().enumerate() {
            p.push(value);
            let mut tmp = nums.clone();
            tmp.remove(i);
            Self::_permute(tmp, res, p);
            p.pop();
        }
    }
}