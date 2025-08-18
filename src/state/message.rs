use crate::{shared::{any_unit::AnyUnit, category::Category}};

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    CategoryChanged(Category),
    FromUnitChanged(AnyUnit),
    ToUnitChanged(AnyUnit),
    SwapUnits,
    Convert,
}
