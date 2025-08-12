#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Meters,
    Kilometers,
    Miles,
    Feet,
}

impl LengthUnit {
    pub const ALL: [LengthUnit; 4] = [
        LengthUnit::Meters,
        LengthUnit::Kilometers,
        LengthUnit::Miles,
        LengthUnit::Feet,
    ];

    pub fn to_meters_factor(self) -> f64 {
        match self {
            LengthUnit::Meters => 1.0,
            LengthUnit::Kilometers => 1_000.0,
            LengthUnit::Miles => 1_609.344,
            LengthUnit::Feet => 0.3048,
        }
    }

    pub fn convert(self, from: f64, to: LengthUnit) -> f64 {
        let in_meters = from * self.to_meters_factor();
        return in_meters / to.to_meters_factor();
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
