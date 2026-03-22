use std::io::{self, Write};

use comfy_table::Table;

fn main() {
    // let total = read_f64("Total:");
    // let tax = read_f64("Tax:");
    // let subtotal = total - tax;
    let subtotal = 69.45;
    println!("Subtotal: {subtotal}");

    // let mut table = Table::new();
    // table.set_header(["Tip %", "Tip", "Total"]);
    // for percentage in [0.15, 0.175, 0.20, 0.25] {
    //     let tip = subtotal * percentage;
    //     table.add_row([
    //         format!("{:.2}%", percentage * 100.0),
    //         format!("${:.2}", tip),
    //         format!("${:.2}", subtotal + tip),
    //     ]);
    // }
    // println!("Suggested tip amounts for a subtotal of ${subtotal}:");
    // println!("{table}");

    let min_total = (subtotal * 1.15_f64).ceil() as u64;
    let max_total = (subtotal * 1.20_f64).floor() as u64;

    let mut round_table = Table::new();
    round_table.set_header(["Tip %", "Tip", "Total"]);
    for total in min_total..=max_total {
        let tip = total as f64 - subtotal;
        let percentage = tip / subtotal * 100.0;
        round_table.add_row([
            format!("{:.2}%", percentage),
            format!("${:.2}", tip),
            format!("${total}.00"),
        ]);
    }
    println!("Round-total tips for a subtotal of ${subtotal}:");
    println!("{round_table}");
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
