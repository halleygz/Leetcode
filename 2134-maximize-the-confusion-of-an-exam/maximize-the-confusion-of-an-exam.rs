impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.chars().collect::<Vec<char>>();
        
        let mut count_t = 0;
        let mut count_f = 0;
        let mut result = 1;
        let mut j = 0;
        
        for i in 0..answer_key.len() {
            if answer_key[i] == 'T' { count_t += 1 }
            if answer_key[i] == 'F' { count_f += 1 }
            
            while std::cmp::min(count_t, count_f) > k {
                if answer_key[j] == 'T' { count_t -= 1 }
                if answer_key[j] == 'F' { count_f -= 1 }
                
                j += 1
            }
            
            result = std::cmp::max(result, i - j + 1);
        }
        
        result as i32
    }
}