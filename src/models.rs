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

impl Category {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Food"          => Ok(Category::Food),
            "Utilities"     => Ok(Category::Utilities),
            "Transport"     => Ok(Category::Transport),
            "Entertainment" => Ok(Category::Entertainment),
            "Healthcare"    => Ok(Category::Healthcare),
            other           => Ok(Category::Other(other.to_string())),
        }
    }
}
