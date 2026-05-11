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
}
