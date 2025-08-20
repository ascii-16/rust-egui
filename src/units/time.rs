use super::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

impl Unit for TimeUnit {
    const ALL: &'static [Self] = &[
        TimeUnit::Seconds,
        TimeUnit::Minutes,
        TimeUnit::Hours,
        TimeUnit::Days,
        TimeUnit::Weeks,
    ];

    fn to_base(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Seconds => value,
            TimeUnit::Minutes => value / 60.0,
            TimeUnit::Hours => value / 3600.0,
            TimeUnit::Days => value / 86_400.0,
            TimeUnit::Weeks => value / 604_800.0,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Seconds => value,
            TimeUnit::Minutes => value * 60.0,
            TimeUnit::Hours => value * 3600.0,
            TimeUnit::Days => value * 86_400.0,
            TimeUnit::Weeks => value * 604_800.0,
        }
    }
}

impl Default for TimeUnit {
    fn default() -> Self {
        TimeUnit::Seconds
    }
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            TimeUnit::Seconds => "Seconds",
            TimeUnit::Minutes => "Minutes",
            TimeUnit::Hours => "Hours",
            TimeUnit::Days => "Days",
            TimeUnit::Weeks => "Weeks",
        };
        write!(f, "{}", name)
    }
}
