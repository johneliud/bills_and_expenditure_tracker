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

    pub fn from_menu(choice: &str, custom: Option<String>) -> Result<Self, String> {
        match choice.trim() {
            "1" => Ok(Category::Food),
            "2" => Ok(Category::Utilities),
            "3" => Ok(Category::Transport),
            "4" => Ok(Category::Entertainment),
            "5" => Ok(Category::Healthcare),
            "6" => match custom {
                Some(name) => Ok(Category::Other(name)),
                None       => Err("Custom category name required".to_string()),
            },
            _ => Err(format!("'{}' is not a valid menu choice (1–6)", choice)),
        }
    }

    pub fn to_file_str(&self) -> String {
        match self {
            Category::Other(name) => name.clone(),
            other                 => format!("{}", other),
        }
    }
}
