impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter()
            .fold(0, |dep, op| {
                match op.as_str() {
                    "../" => {
                        match dep {
                            0 => dep,
                            _ => dep - 1,
                        }
                    }
                    "./" => dep, 
                    _ => dep + 1,  
                }
             })
    }
}