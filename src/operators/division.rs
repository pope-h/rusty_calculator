pub fn divide(first: i32, second: i32) {
    if second == 0 {
        println!("Error: Division by zero is not allowed!");
    } else {
        let result: f64 = (first as f64) / (second as f64);
        if result.is_infinite() {
            println!("Error: Result is too large to represent!");
        } else {
            println!("Result: {:.2}", result);
        }
    }
}