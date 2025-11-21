impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|val| val[0] );
        let mut res = Vec::with_capacity(intervals.len());
        let last = intervals.iter().fold((intervals[0][0], intervals[0][1]),|(l,r), curent| {
            if curent[0] > r {
                res.push(vec![l,r]);
                return (curent[0], curent[1])
            }
            (l, r.max(curent[1]))
        });
        res.push(vec![last.0, last.1]);
        res
    }
}