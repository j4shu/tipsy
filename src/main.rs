use std::io::{self, Write};

fn main() {
    let total = read_f64("Total:");
    let tax = read_f64("Tax:");
    let subtotal = total - tax;
    println!("Subtotal: {subtotal}");

    let min_total = (subtotal * 1.15_f64).ceil() as u64;
    let max_total = (subtotal * 1.20_f64).floor() as u64;

    println!("{:<10} {:<10} {:<10}", "Tip %", "Tip", "Total");
    for total in min_total..=max_total {
        let tip = total as f64 - subtotal;
        let percentage = tip / subtotal * 100.0;
        println!(
            "{:<10} {:<10} ${total}.00",
            format!("{percentage:.2}%"),
            format!("${tip:.2}")
        );
    }
}

fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    loop {
        input.clear();

        // use print! instead of println! to keep the prompt and input on the same line
        print!("{prompt} ");

        // flush stdout so the prompt appears before read_line blocks for input
        io::stdout().flush().unwrap();

        // read input
        io::stdin().read_line(&mut input).expect("Failed to parse");

        // parse input as f64 and handle errors
        match input.trim().parse::<f64>() {
            Ok(num) if num < 0.0 => {
                println!("Input cannot be negative");
            }
            Ok(num) => break num,
            Err(_) => {
                println!("Please input a valid number");
            }
        }
    }
}
