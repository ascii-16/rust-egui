use iced::{
    widget::{column, container, row, text, Space}, Element, Length, Task
};
use rust_egui::{
    shared::{any_unit::AnyUnit, category::Category},
    state::message::Message,
    ui::{category_selector, converter_view},
};

pub struct MyApp {
    pub category: Category,
    pub input_value: String,
    pub from_unit: AnyUnit,
    pub to_unit: AnyUnit,
    pub units: Vec<AnyUnit>,
    pub result: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        let category = Category::Length;
        let units = AnyUnit::items_by_category(category);
        Self {
            category,
            input_value: String::new(),
            from_unit: units[0],
            to_unit: units[1],
            units,
            result: 0.0,
        }
    }
}

impl MyApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(val) => self.input_value = val,
            Message::CategoryChanged(val) => {
                self.category = val;
                self.units = AnyUnit::items_by_category(val);
                self.from_unit = self.units[0];
                self.to_unit = self.units[1];
            }
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

    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("Unit Converter").size(24),
                row![
                    category_selector::category_selector(self.category),
                    Space::with_width(Length::Fixed(16.0)),
                    converter_view::converter_view(
                        &self.input_value,
                        &self.from_unit,
                        &self.to_unit,
                        &self.units,
                        self.result
                    )
                ]
            ]
            .spacing(20),
        )
        .padding(20)
        .into()
    }
}
