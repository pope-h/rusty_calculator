use crate::operators::{addition::add, division::divide, multiplication::multiply, subtraction::subtract};

pub fn calculate(first: i32, second: i32, operator: char) {
    match operator {
        '+' => {
            add(first, second);
        },
        '-' => {
            subtract(first, second);
        },
        '*' => {
            multiply(first, second);
        },
        '/' => {
            divide(first, second);
        },
        _ => println!("Invalid operator!")
    }
}