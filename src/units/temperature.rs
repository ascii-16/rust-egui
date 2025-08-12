use super::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Unit for TemperatureUnit {
    const ALL: &'static [Self] = &[
        TemperatureUnit::Celsius,
        TemperatureUnit::Fahrenheit,
        TemperatureUnit::Kelvin,
    ];

    fn to_base(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => value,
            TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            TemperatureUnit::Kelvin => value - 273.15,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => value,
            TemperatureUnit::Fahrenheit => value * 9.0 / 5.0 + 32.0,
            TemperatureUnit::Kelvin => value + 273.15,
        }
    }
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        TemperatureUnit::Celsius
    }
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "Kelvin",
        };
        write!(f, "{}", name)
    }
}
