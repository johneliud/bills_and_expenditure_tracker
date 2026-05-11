use std::io::{self, Write};
use crate::models::Category;

pub fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn read_amount() -> Result<f64, String> {
    let raw = prompt("Amount");
    raw.parse::<f64>()
       .map_err(|e| format!("Invalid amount '{}': {}", raw, e))
}
