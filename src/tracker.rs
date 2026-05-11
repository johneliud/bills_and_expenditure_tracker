use std::fs::{File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::models::{Bill, Category};

pub struct BillTracker {
    bills:     Vec<Bill>,
    next_id:   u32,
    data_file: String,
}

impl BillTracker {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let bills   = Self::load(path)?;
        let next_id = bills.iter().map(|b| b.id).max().unwrap_or(0) + 1;
        Ok(BillTracker {
            bills,
            next_id,
            data_file: path.to_string(),
        })
    }

    fn load(path: &str) -> Result<Vec<Bill>, io::Error> {
        if !Path::new(path).exists() {
            return Ok(Vec::new());
        }
        let file   = File::open(path)?;
        let reader = BufReader::new(file);
        let bills  = reader
            .lines()
            .filter_map(|result| {
                let line = result.ok()?;
                if line.trim().is_empty() { return None; }
                Bill::from_csv_row(&line).ok()
            })
            .collect();
        Ok(bills)
    }

    fn save(&self) -> Result<(), io::Error> {
        let file       = File::create(&self.data_file)?;
        let mut writer = BufWriter::new(file);
        for bill in &self.bills {
            writeln!(writer, "{}", bill.to_csv_row())?;
        }
        Ok(())
    }

    pub fn add(
        &mut self,
        name: String,
        amount: f64,
        category: Category,
    ) -> Result<&Bill, io::Error> {
        let bill = Bill { id: self.next_id, name, amount, category };
        self.next_id += 1;
        self.bills.push(bill);
        self.save()?;
        Ok(self.bills.last().unwrap())
    }

    pub fn view_all(&self) -> &[Bill] {
        &self.bills
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Bill> {
        self.bills.iter().find(|b| b.id == id)
    }

    pub fn total(&self) -> f64 {
        self.bills.iter().map(|b| b.amount).sum()
    }
}
