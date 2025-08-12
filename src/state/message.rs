#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    FromUnitChanged(crate::categories::length::LengthUnit),
    ToUnitChanged(crate::categories::length::LengthUnit),
    SwapUnits,
    Convert,
}
