pub trait Unit: Sized + Clone + std::fmt::Display + PartialEq + 'static {
    const ALL: &'static [Self];

    /// Logic to convert to the base unit
    /// Eg: For length, converting to meters
    fn to_base(&self, value: f64) -> f64;

    /// Logic to coverting from the base unit
    fn from_base(&self, value: f64) -> f64;

    fn convert(&self, value: f64, to: &Self) -> f64 {
        // Take any unit as base (Eg: meters in length)
        // Then convert value to base unit first and then to desired unit
        let in_base = self.to_base(value);
        to.from_base(in_base)
    }
}
