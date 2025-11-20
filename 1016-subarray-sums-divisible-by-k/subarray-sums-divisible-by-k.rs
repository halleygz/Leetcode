impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut mod_group = vec![0; k as usize];
        mod_group[0] += 1;
        nums.into_iter()
            .map(|n| (n % k) + k)
            .scan(0, |p, n| {
                *p = (*p + n) % k;
                Some(*p as usize)
            })
            .scan(mod_group, |group, prefix| {
                group[prefix] += 1;
                Some(group[prefix] - 1)
            })
            .sum()
    }
}