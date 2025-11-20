use std::collections::HashMap;
impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let len_n: usize = arr.len();
    let mut answer: Vec<i64> = vec![0; len_n];
    let mut idx_sum: HashMap<i32, i64> = HashMap::new();
    let mut num_frq: HashMap<i32, i32> = HashMap::new();

    for (idx, &num) in arr.iter().enumerate() {
        answer[idx] += (idx as i64) * (*num_frq.entry(num).or_insert(0) as i64)
            - (*idx_sum.entry(num).or_insert(0) as i64);
        *idx_sum.entry(num).or_default() += idx as i64;
        *num_frq.entry(num).or_default() += 1;
    }

    idx_sum.clear();
    num_frq.clear();

    for (idx, &num) in arr.iter().rev().enumerate() {
        let idx = len_n - idx - 1;
        answer[idx] += (len_n - idx - 1) as i64 * (*num_frq.entry(num).or_insert(0) as i64)
            - (*idx_sum.entry(num).or_insert(0) as i64);
        *idx_sum.entry(num).or_default() += (len_n - idx -1) as i64;
        *num_frq.entry(num).or_default() +=1;
    }
    
    answer
    }
}