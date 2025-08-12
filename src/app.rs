use iced::{Element, Task};
use rust_egui::{state::message::Message, ui::converter_view, units::{unit::Unit}};

#[derive(Default)]
pub struct MyApp <U: Unit + 'static> {
    pub input_value: String,
    pub from_unit: U,
    pub to_unit: U,
    pub result: f64,
}

impl<U: Unit> MyApp<U> {
    pub fn update(&mut self, message: Message<U>) -> Task<Message<U>> {
        match message {
            Message::InputChanged(val) => self.input_value = val,
            Message::FromUnitChanged(unit) => self.from_unit = unit,
            Message::ToUnitChanged(unit) => self.to_unit = unit,
            Message::SwapUnits => std::mem::swap(&mut self.from_unit, &mut self.to_unit),
            Message::Convert => {
                if let Ok(v) = self.input_value.parse::<f64>() {
                    self.result = self.from_unit.convert(v, &self.to_unit);
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message<U>> {
        converter_view::converter_view(&self.input_value, self.from_unit.clone(), self.to_unit.clone(), self.result)
    }
}
