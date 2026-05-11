use std::fmt;

#[derive(Debug, Clone)]
pub enum Category {
    Food,
    Utilities,
    Transport,
    Entertainment,
    Healthcare,
    Other(String),
}
