use iced::widget::{button, column, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::categories::length::LengthUnit;
use crate::state::message::Message;

pub fn converter_view(
    input_value: &str,
    from_unit: LengthUnit,
    to_unit: LengthUnit,
    result: f64,
) -> Element<'static, Message> {
    column![
        text("Unit Converter").size(24),
        row![
            text_input("Enter value", input_value)
                .on_input(Message::InputChanged)
                .width(Length::Fixed(150.0)),
            pick_list(
                &LengthUnit::ALL[..],
                Some(from_unit),
                Message::FromUnitChanged
            )
            .width(Length::Fixed(120.0))
        ]
        .spacing(10)
        .align_y(Alignment::Center),

        row![
            button("⇆ Swap").on_press(Message::SwapUnits)
        ]
        .spacing(5)
        .align_y(Alignment::Center),

        row![
            text(format!("{:.4}", result)).size(20),
            pick_list(&LengthUnit::ALL[..], Some(to_unit), Message::ToUnitChanged)
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
