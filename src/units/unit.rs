pub trait Unit: Sized + Clone + std::fmt::Display + PartialEq + 'static {
    const ALL: &'static [Self];

    fn to_base(&self, value: f64) -> f64;

    fn from_base(&self, value: f64) -> f64;

    fn convert(&self, value: f64, to: &Self) -> f64 {
        let in_base = self.to_base(value);
        to.from_base(in_base)
    }
}
