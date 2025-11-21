use std::collections::HashSet;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut res: HashSet<String> = HashSet::new();
        let mut hs: HashSet<String> = HashSet::new();
        let n = s.len();
        if n < 10 { return vec![] }
        for i in 0..=(n-10) {
        let substr = s[i..(i+10)].to_string();
        if hs.contains(&substr) { res.insert(substr.clone()); }
        hs.insert(substr);
        }
        let resv: Vec<String> = res.into_iter().collect();
        resv
    }
}