impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut ss: Vec<_> = nums.iter().map(i32::to_string).collect();
    ss.sort_unstable_by(|a, b| // ba <> ab
             b.bytes().chain(a.bytes())
        .cmp(a.bytes().chain(b.bytes()))
    );
    let res: String = ss.into_iter().collect();
    if res.starts_with("0") { "0".into() } else { res }
    }
}