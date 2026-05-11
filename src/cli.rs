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

pub fn read_category() -> Result<Category, String> {
    println!("  [1] Food  [2] Utilities  [3] Transport  [4] Entertainment  [5] Healthcare  [6] Other");
    let choice = prompt("Category");
    let custom = if choice.trim() == "6" {
        Some(prompt("Custom category name"))
    } else {
        None
    };
    Category::from_menu(&choice, custom)
}

pub fn read_optional_string(label: &str) -> Option<String> {
    let val = prompt(&format!("{} (blank = keep)", label));
    if val.is_empty() { None } else { Some(val) }
}

pub fn read_optional_amount() -> Option<f64> {
    let raw = prompt("Amount (blank = keep)");
    if raw.is_empty() {
        None
    } else {
        raw.parse::<f64>().ok()
    }
}

pub fn read_optional_category() -> Option<Category> {
    let skip = prompt("Change category? [y/N]");
    if skip.trim().to_lowercase() != "y" {
        return None;
    }
    match read_category() {
        Ok(cat) => Some(cat),
        Err(_)  => None,
    }
}
