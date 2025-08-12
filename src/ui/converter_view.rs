use iced::widget::{button, column, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::state::message::Message;
use crate::units::unit::Unit;

pub fn converter_view<U: Unit>(
    input_value: &str,
    from_unit: U,
    to_unit: U,
    result: f64,
) -> Element<'static, Message<U>> {
    column![
        text("Unit Converter").size(24),
        row![
            text_input("Enter value", input_value)
                .on_input(Message::InputChanged)
                .width(Length::Fixed(150.0)),
            pick_list(&U::ALL[..], Some(from_unit), Message::FromUnitChanged)
                .width(Length::Fixed(120.0))
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![button("⇆ Swap").on_press(Message::SwapUnits)]
            .spacing(5)
            .align_y(Alignment::Center),
        row![
            text(format!("{:.4}", result)).size(20),
            pick_list(&Unit::ALL[..], Some(to_unit), Message::ToUnitChanged)
                .width(Length::Fixed(120.0))
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        button("Convert").on_press(Message::Convert)
    ]
    .spacing(15)
    .align_x(Alignment::Center)
    .into()
}
