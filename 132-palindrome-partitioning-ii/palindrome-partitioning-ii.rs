impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }

        let s: Vec<char> = s.chars().collect();
        let mut cuts: Vec<i32> = (0..n as i32).collect();

        for i in 0..n {
            let mut l = i;
            let mut r = i;
            while l >= 0 && r < n && s[l] == s[r] {
                cuts[r] = if l == 0 { 0 } else { cuts[r].min(cuts[l - 1] + 1) };
                if l == 0 { break; }
                l -= 1;
                r += 1;
            }

            let mut l = i;
            let mut r = i + 1;
            while l >= 0 && r < n && s[l] == s[r] {
                cuts[r] = if l == 0 { 0 } else { cuts[r].min(cuts[l - 1] + 1) };
                if l == 0 { break; }
                l -= 1;
                r += 1;
            }
        }

        cuts[n - 1]
    }
}