impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;

        let len_g = g.len();
        let len_s = s.len();

        g.sort();
        s.sort();

        let mut i = 0; // index for children (greed)
        let mut j = 0; // index for cookie size 
        
        let mut hit = 0;
        while i < len_g && j < len_s {

            while j < len_s {
                if s[j] >= g[i] {
                    hit += 1;
                    j += 1;
                    break;
                }
                j += 1;
            }
            i += 1;
        }

        hit
    }
}