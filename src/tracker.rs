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
}
