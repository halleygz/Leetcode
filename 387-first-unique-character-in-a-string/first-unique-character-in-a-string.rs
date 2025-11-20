impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_vec = vec![0; 26];
        s.chars().for_each(|c| { char_vec[c as usize - 'a' as usize] += 1;});
        for (i, c) in s.char_indices() {
            if char_vec[c as usize - 'a' as usize] == 1 {
                return  i as i32;
            }
        }
        -1
    }
}