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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Food          => write!(f, "Food"),
            Category::Utilities     => write!(f, "Utilities"),
            Category::Transport     => write!(f, "Transport"),
            Category::Entertainment => write!(f, "Entertainment"),
            Category::Healthcare    => write!(f, "Healthcare"),
            Category::Other(name)   => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bill {
    pub id:       u32,
    pub name:     String,
    pub amount:   f64,
    pub category: Category,
}

impl Bill {
    pub fn to_csv_row(&self) -> String {
        format!("{}|{}|{:.2}|{}", self.id, self.name, self.amount, self.category.to_file_str())
    }
    }
}
