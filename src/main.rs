pub mod operators;
pub mod separate_inputs;
pub mod compute;
use std::io;

fn main() {
    println!("Please input calculation (e.g., 23 + 9):");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Trim whitespace
    let input = input.trim();
    
    // Find the first operator and its position
    let mut operator = None;
    let mut operator_pos = None;
    
    for (pos, c) in input.chars().enumerate() {
        if "+-*/".contains(c) {
            operator = Some(c);
            operator_pos = Some(pos);
            break;  // Only take the first operator we find
        }
    }

    separate_inputs::separate_inputs(input, operator, operator_pos);
}