use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Length,
    Temperature,
    Weight,
}

impl Category {
    pub const ALL: [Category; 3] = [Category::Length, Category::Temperature, Category::Weight];
}

impl fmt::Display for Category  {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Category::Length => "Length",
            Category::Temperature => "Temperature",
            Category::Weight => "Weight",
        };
        write!(f, "{}", name)
    }
}

impl Default for Category {
    fn default() -> Self {
        Category::Length
    }
}
