use std::fmt;
use crate::{
    shared::category::Category,
    units::{length::LengthUnit, temperature::TemperatureUnit, unit::Unit, weight::WeightUnit},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyUnit {
    Length(LengthUnit),
    Temperature(TemperatureUnit),
    Weight(WeightUnit),
}

impl AnyUnit {
    pub fn category(&self) -> Category {
        match self {
            AnyUnit::Length(_) => Category::Length,
            AnyUnit::Temperature(_) => Category::Temperature,
            AnyUnit::Weight(_) => Category::Weight,
        }
    }

    pub fn items_by_category(category: Category) -> Vec<AnyUnit> {
        match category {
            Category::Length => LengthUnit::ALL
                .iter()
                .copied()
                .map(AnyUnit::Length)
                .collect(),
            Category::Temperature => TemperatureUnit::ALL
                .iter()
                .copied()
                .map(AnyUnit::Temperature)
                .collect(),
            Category::Weight => WeightUnit::ALL
                .iter()
                .copied()
                .map(AnyUnit::Weight)
                .collect(),
        }
    }

    pub fn to_base(&self, value: f64) -> f64 {
        match self {
            AnyUnit::Length(u) => u.to_base(value),
            AnyUnit::Temperature(u) => u.to_base(value),
            AnyUnit::Weight(u) => u.to_base(value),
        }
    }

    pub fn from_base(&self, value: f64) -> f64 {
        match self {
            AnyUnit::Length(u) => u.from_base(value),
            AnyUnit::Temperature(u) => u.from_base(value),
            AnyUnit::Weight(u) => u.to_base(value),
        }
    }

    pub fn convert(&self, value: f64, to: &Self) -> f64 {
        // Take any unit as base (Eg: meters in length)
        // Then convert value to base unit first and then to desired unit
        let in_base = self.to_base(value);
        to.from_base(in_base)
    }
}

impl fmt::Display for AnyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyUnit::Length(u) => write!(f, "{}", u),
            AnyUnit::Temperature(u) => write!(f, "{}", u),
            AnyUnit::Weight(u) => write!(f, "{}", u),
        }
    }
}

impl Default for AnyUnit {
    fn default() -> Self {
        AnyUnit::Length(LengthUnit::Meters)
    }
}
