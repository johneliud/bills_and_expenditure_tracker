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

fn handle_add(tracker: &mut BillTracker) {
    let name = cli::prompt("\nName");
    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }
    let amount = match cli::read_amount() {
        Ok(a)  => a,
        Err(e) => { println!("Error: {}", e); return; }
    };
    let category = match cli::read_category() {
        Ok(c)  => c,
        Err(e) => { println!("Error: {}", e); return; }
    };
    match tracker.add(name, amount, category) {
        Ok(bill) => println!("\nAdded: {}", bill),
        Err(e)   => println!("\nFailed to save: {}", e),
    }
}

fn handle_view(tracker: &BillTracker) {
    let bills = tracker.view_all();
    if bills.is_empty() {
        println!("\nNo bills recorded.");
        return;
    }
    println!();
    for bill in bills {
        println!("  {}", bill);
    }
    println!("\n  --- {} bill(s) ---", bills.len());
}

fn handle_update(tracker: &mut BillTracker) {
    let id_str = cli::prompt("Bill ID to update");
    let id = match id_str.trim().parse::<u32>() {
        Ok(n)  => n,
        Err(_) => { println!("Invalid ID."); return; }
    };

    if tracker.find_by_id(id).is_none() {
        println!("\nNo bill with ID {}.", id);
        return;
    }

    let name     = cli::read_optional_string("Name");
    let amount   = cli::read_optional_amount();
    let category = cli::read_optional_category();

    match tracker.edit(id, name, amount, category) {
        Ok(Some(bill)) => println!("\nUpdated: {}", bill),
        Ok(None)       => println!("\nBill not found."),
        Err(e)         => println!("\nSave error: {}", e),
    }
}

fn handle_remove(tracker: &mut BillTracker) {
    let id_str = cli::prompt("Bill ID to remove");
    let id = match id_str.trim().parse::<u32>() {
        Ok(n)  => n,
        Err(_) => { println!("Invalid ID."); return; }
    };
    match tracker.remove(id) {
        Ok(Some(bill)) => println!("\nRemoved: {}", bill),
        Ok(None)       => println!("\nNo bill with ID {}.", id),
        Err(e)         => println!("\nSave error: {}", e),
    }
}

fn handle_total(tracker: &BillTracker) {
    let bills = tracker.view_all();
    if bills.is_empty() {
        println!("\nNo bills recorded.");
    } else {
        println!("\nTotal: KES{:.2}", tracker.total());
    }
}
