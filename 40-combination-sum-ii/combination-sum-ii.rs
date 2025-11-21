impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn solve(i: usize, target: i32, candidates: &Vec<i32>, x: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if i == candidates.len() || target <= 0 {
                if target == 0 {ans.push(x.to_vec());}
                return;
            }
            for j in i..candidates.len() {
                if j != i && candidates[j] == candidates[j-1] {continue;}
                x.push(candidates[j]);
                solve(j+1, target-candidates[j], candidates, x, ans);
                x.pop();
            }
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();
        candidates.sort_unstable();
        solve(0, target, &candidates, &mut vec![], &mut ans);
        ans
    }
}