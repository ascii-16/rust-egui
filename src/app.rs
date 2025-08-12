use iced::{Element, Task};
use rust_egui::{categories::length::LengthUnit, state::message::Message, ui::converter_view};

#[derive(Default)]
pub struct MyApp {
    pub input_value: String,
    pub from_unit: LengthUnit,
    pub to_unit: LengthUnit,
    pub result: f64,
}

impl MyApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(val) => self.input_value = val,
            Message::FromUnitChanged(unit) => self.from_unit = unit,
            Message::ToUnitChanged(unit) => self.to_unit = unit,
            Message::SwapUnits => std::mem::swap(&mut self.from_unit, &mut self.to_unit),
            Message::Convert => {
                if let Ok(v) = self.input_value.parse::<f64>() {
                    self.result = self.from_unit.convert(v, self.to_unit);
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        converter_view::converter_view(&self.input_value, self.from_unit, self.to_unit, self.result)
    }
}
