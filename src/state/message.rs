use crate::units::unit::Unit;

#[derive(Debug, Clone)]
pub enum Message<U: Unit> {
    InputChanged(String),
    FromUnitChanged(U),
    ToUnitChanged(U),
    SwapUnits,
    Convert,
}
