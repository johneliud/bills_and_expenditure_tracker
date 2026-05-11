mod models;
mod tracker;
mod cli;

use tracker::BillTracker;

enum Command {
    Add,
    View,
    Remove,
    Update,
    Total,
    Quit,
    Unknown(String),
}

impl Command {
    fn from_input(s: &str) -> Self {
        match s.trim() {
            "1" => Command::Add,
            "2" => Command::View,
            "3" => Command::Remove,
            "4" => Command::Update,
            "5" => Command::Total,
            "0" | "q" => Command::Quit,
            other => Command::Unknown(other.to_string()),
        }
    }
}

fn print_menu() {
    println!();
    println!("== Manage Bills ==\n");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total");
    println!("\nPress '0', 'q' or 'CTRL + C' to exit");
}
