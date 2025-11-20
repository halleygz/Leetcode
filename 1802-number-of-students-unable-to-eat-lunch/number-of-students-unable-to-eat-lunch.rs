impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut sandwiches: std::collections::VecDeque<_> = sandwiches.into();
        let mut students = students.into_iter().fold(vec![0; 2], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });
        while !sandwiches.is_empty() && students[0] >= 0 && students[1] >= 0 {
            if students[*sandwiches.front().unwrap() as usize] == 0 { break; }
            students[sandwiches.pop_front().unwrap() as usize] -= 1;
        }
        students[0] + students[1]
    }
}