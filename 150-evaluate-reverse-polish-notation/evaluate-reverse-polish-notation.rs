use std::ops::{Add, Sub, Mul, Div};
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        let binary_op = |f: fn(_, _) -> _, stack: &mut Vec<_>| {
            let (o2, o1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(f(o1, o2))
        };

        for token in tokens {
            match token.as_str() {
                "+" => binary_op(i32::add, &mut stack),
                "-" => binary_op(i32::sub, &mut stack),
                "*" => binary_op(i32::mul, &mut stack),
                "/" => binary_op(i32::div, &mut stack),
                number => stack.push(number.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}