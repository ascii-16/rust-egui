use super::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Meters,
    Kilometers,
    Miles,
    Feet,
}

impl Unit for LengthUnit {
    const ALL: &'static [Self] = &[
        LengthUnit::Meters,
        LengthUnit::Kilometers,
        LengthUnit::Miles,
        LengthUnit::Feet,
    ];

    fn to_base(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Meters => value * 1.0,
            LengthUnit::Kilometers => value * 1_000.0,
            LengthUnit::Miles => value * 1_609.344,
            LengthUnit::Feet => value * 0.3048,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Meters => value / 1.0,
            LengthUnit::Kilometers => value / 1_000.0,
            LengthUnit::Miles => value / 1_609.344,
            LengthUnit::Feet => value / 0.3048,
        }
    }
}

impl Default for LengthUnit {
    fn default() -> Self {
        LengthUnit::Meters
    }
}

impl std::fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            LengthUnit::Meters => "Meters",
            LengthUnit::Kilometers => "Kilometers",
            LengthUnit::Miles => "Miles",
            LengthUnit::Feet => "Feet",
        };
        write!(f, "{}", name)
    }
}
