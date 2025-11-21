impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mana = Self::manacher(&s);
        let mut ans = Vec::new();
        Self::backtrack(&mut vec![0], &mut ans, &mana, &s);
        ans
    }

    fn manacher(s: &str) -> Vec<usize> {
        let mut v = vec!['#'; s.len() * 2 + 1];
        v[0] = '$';
        *v.last_mut().unwrap() = '^';
        for i in 0..s.len() {
            v[i * 2 + 1] = s.as_bytes()[i] as char;
        }
        let mut p = vec![0; v.len()];
        let mut l = 1;
        let mut r = 1;
        for i in 1..p.len() - 1 {
            p[i] = (r - i).min(p[l + r - i]).max(0);
            while v[i - p[i]] == v[i + p[i]] {
                p[i] += 1;
            }
            if i + p[i] > r {
                l = i - p[i];
                r = i + p[i];
            }
        }
        p
    }

    fn backtrack(curr: &mut Vec<usize>, ans: &mut Vec<Vec<String>>,
    mana: &Vec<usize>, s: &String) {
        let i = *curr.last().unwrap();
        if i == s.len() {
            ans.push(curr.windows(2)
                .map(|w| s[w[0]..w[1]].to_owned())
                .collect());
        }
        for j in i..s.len() {
            if Self::is_palin(i, j, mana) {
                curr.push(j + 1);
                Self::backtrack(curr, ans, mana, s);
                curr.pop();
            }
        }
    }
    
    fn is_palin(i: usize, j: usize, mana: &Vec<usize>) -> bool {
        mana[i + j + 1] > j - i
    }
}