// This script splits the input at the operator position
use crate::compute;

pub fn separate_inputs(input: &str, operator: Option<char>, operator_pos: Option<usize>) {
    if let (Some(op), Some(pos)) = (operator, operator_pos) {
        let first_str = &input[..pos].trim();   // if the pos=3, it takes the char between 0 and 2 and trim off the spaces
        let second_str = &input[pos + 1..].trim();  // using pos=3, it takes the char between 4 and the end of the string and trim off the spaces

        match (first_str.parse::<i32>(), second_str.parse::<i32>()) {
            (Ok(first), Ok(second)) => {
                println!("First number: {}", first);
                println!("Operator: {}", op);
                println!("Second number: {}", second);

                compute::calculate(first, second, op);
            },
            _ => println!("Invalid input! Please enter two numbers separated by an operator (+, -, *, /)")
        }
    } else {
        println!("Invalid input! No operator found.");
    }
}