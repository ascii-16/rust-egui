use super::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightUnit {
    Gram,
    Kilogram,
    Pound,
    Milligram,
}

impl Unit for WeightUnit {
    const ALL: &'static [Self] = &[
        WeightUnit::Milligram,
        WeightUnit::Gram,
        WeightUnit::Kilogram,
        WeightUnit::Pound,
    ];

    fn to_base(&self, value: f64) -> f64 {
        match self {
            WeightUnit::Gram => value,
            WeightUnit::Kilogram => value * 1_000.0,
            WeightUnit::Pound => value * 453.59237,
            WeightUnit::Milligram => value / 1_000.0,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            WeightUnit::Gram => value,
            WeightUnit::Kilogram => value / 1_000.0,
            WeightUnit::Pound => value / 453.59237,
            WeightUnit::Milligram => value * 1_000.0,
        }
    }
}

impl Default for WeightUnit {
    fn default() -> Self {
        WeightUnit::Gram
    }
}

impl std::fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            WeightUnit::Gram => "Gram",
            WeightUnit::Kilogram => "Kilogram",
            WeightUnit::Pound => "Pound",
            WeightUnit::Milligram => "Milligram",
        };
        write!(f, "{}", name)
    }
}