use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Length,
    Temperature,
    Weight,
    Time,
}

impl Category {
    pub const ALL: [Category; 4] = [Category::Length, Category::Temperature, Category::Weight, Category::Time];
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Category::Length => "Length",
            Category::Temperature => "Temperature",
            Category::Weight => "Weight",
            Category::Time => "TIme",
        };
        write!(f, "{}", name)
    }
}

impl Default for Category {
    fn default() -> Self {
        Category::Length
    }
}
