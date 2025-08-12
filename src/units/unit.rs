pub trait Unit: Sized + Clone + std::fmt::Display + PartialEq + 'static {
    const ALL: &'static [Self];

    fn to_base_factor(&self) -> f64;

    fn convert(&self, value: f64, to: &Self) -> f64 {
        let in_base = value * self.to_base_factor();
        in_base / to.to_base_factor()
    }
}
