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
