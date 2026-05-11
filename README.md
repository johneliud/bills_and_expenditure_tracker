# Bills & Expenditure Tracker

A command-line application to track bills and expenditures, written in Rust.
Built as a learning project to practice core Rust concepts: enums, `Option`, `Result`, `match`, iterators, ownership, borrowing, and mutability.

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, rustc 1.85+)
- No external dependencies - stdlib only

---

## Clone & Run

```bash
# Clone the repository
git clone https://github.com/johneliud/bills_and_expenditure_tracker.git
cd bills_and_expenditure_tracker

# Build and run
cargo run
```

Or build a release binary first:

```bash
cargo build --release
./target/release/bills_and_expenditure_tracker
```

---

## Usage

On launch you will see a numbered menu. Enter the number and press Enter:

```
== Manage Bills ==
1. Add bill
2. View bills
3. Remove bill
4. Update bill
5. Bill total
Enter selection:
```

---

## Supported Options

| # | Action | Description |
|---|--------|-------------|
| `1` | Add bill | Prompt for name, amount, and category, then save a new bill |
| `2` | View bills | List every recorded bill with ID, name, amount, and category |
| `3` | Remove bill | Delete a bill by its ID |
| `4` | Update bill | Update one or more fields of an existing bill by its ID |
| `5` | Bill total | Print the sum of all bill amounts |

Enter `0`, `q`, or `quit` to exit.

---

## Adding a Bill

```
Enter selection: 1
Name: Electricity
Amount: 120.50
  [1] Food  [2] Utilities  [3] Transport
  [4] Entertainment  [5] Healthcare  [6] Other
Category: 2
Added: #1: Electricity - KES120.50 (Utilities)
```

Choosing category `6` prompts for a custom name:

```
Category: 6
Custom category name: Streaming
Added: #2: Netflix - KES15.99 (Streaming)
```

---

## Viewing Bills

```
Enter selection: 2

  #1: Electricity - KES120.50 (Utilities)
  #2: Netflix - KES15.99 (Streaming)
  #3: Groceries - KES89.00 (Food)
  --- 3 bill(s) ---
```

---

## Updating a Bill

Only the fields you fill in are changed. Leave a field blank to keep its current value.

```
Enter selection: 4
Bill ID to update: 3
Name (blank = keep): Groceries & Supplies
Amount (blank = keep):
Change category? [y/N]: n
Updated: #3: Groceries & Supplies - KES89.00 (Food)
```

---

## Removing a Bill

```
Enter selection: 3
Bill ID to remove: 2
Removed: #2: Netflix - KES15.99 (Streaming)
```

---

## Bill Total

```
Enter selection: 5
Total: KES209.50
```

---

## Data Persistence

Bills are saved to `bills.dat` in the current working directory automatically after every change. The file uses a simple pipe-delimited format - one bill per line:

```
1|Electricity|120.50|Utilities
3|Groceries & Supplies|89.00|Food
```

The file is created on the first add and reloaded on every startup. You can inspect or back it up with any text editor.

---

## Supported Categories

| # | Name |
|---|------|
| 1 | Food |
| 2 | Utilities |
| 3 | Transport |
| 4 | Entertainment |
| 5 | Healthcare |
| 6 | Other (custom label) |

---

## Project Structure

```
src/
  main.rs      Command enum, menu loop, handler functions
  models.rs    Bill struct, Category enum, Display and CSV impls
  tracker.rs   BillTracker - CRUD operations and file persistence
  cli.rs       stdin prompt helpers (prompt, read_amount, read_category, ...)
bills.dat      Auto-generated data file (created after first add)
```

---

## Rust Concepts Demonstrated

| Concept | Location |
|---------|----------|
| `enum` with unit variants | `Category`, `Command` in `main.rs` |
| `enum` tuple variant | `Category::Other(String)`, `Command::Unknown(String)` |
| `Option<&Bill>` - borrow on find | `tracker::find_by_id` |
| `Option<Bill>` - move out of `Vec` | `tracker::remove` |
| `Option<T>` params for partial update | `tracker::edit`, `cli::read_optional_*` |
| `.ok()` to convert `Result` to `Option` | `cli::read_optional_amount` |
| `Result<T, E>` for parse errors | `models::from_csv_row`, `cli::read_amount` |
| `Result<T, E>` for file I/O | `tracker::load`, `save`, `add`, `edit`, `remove` |
| `?` operator for error propagation | throughout `tracker.rs` |
| `.map_err()` to convert error types | `models::from_csv_row`, `cli::read_amount` |
| `match` for command dispatch | `main.rs` main loop |
| `match` for error handling | every `Result`/`Option` call site |
| `iter().find()` | `tracker::find_by_id` |
| `iter().filter().collect()` | `tracker::filter_by_category` |
| `iter().map().sum()` | `tracker::total` |
| `iter().position()` | `tracker::edit`, `tracker::remove` |
| `filter_map()` | `tracker::load` - skips malformed lines |
| `&[Bill]` slice borrow | `tracker::view_all` - zero allocation |
| `&mut self` / mutability | `add`, `edit`, `remove` |
| Ownership transfer into function | `handle_add` moves `name`, `amount`, `category` |
| `&str` vs `String` | `cli::prompt` - borrows label, returns owned `String` |
