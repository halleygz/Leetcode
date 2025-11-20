impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = tickets[k];
        tickets
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + match i > k {
                false => v.min(n),
                true => v.min(n - 1)
            })
    }
}