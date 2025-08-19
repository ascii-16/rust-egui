use iced::border::Radius;
use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length, Theme};

use crate::shared::any_unit::AnyUnit;
use crate::state::message::Message;

// Borrow is completed when this function returns Element
// so need to specify a lifetime ('a here)
pub fn converter_view<'a>(
    input_value: &'a str,
    from_unit: &'a AnyUnit,
    to_unit: &'a AnyUnit,
    units: &'a [AnyUnit],
    result: f64,
) -> Element<'a, Message> {
    container(
        column![
            row![
                text_input("Enter value", input_value)
                    .on_input(Message::InputChanged)
                    .width(iced::Length::Fixed(150.0)),
                pick_list(units, Some(*from_unit), Message::FromUnitChanged)
                    .width(iced::Length::Fixed(120.0))
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![button("⇆ Swap").on_press(Message::SwapUnits)]
                .spacing(5)
                .align_y(Alignment::Center),
            row![
                text(format!("{:.4}", result)).size(20),
                pick_list(units, Some(*to_unit), Message::ToUnitChanged)
                    .width(Length::Fixed(120.0))
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            button("Convert").on_press(Message::Convert)
        ]
        .spacing(15)
        .align_x(Alignment::Start),
    )
    .style(|_theme: &Theme| container::Style {
        border: iced::Border {
            color: iced::Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: Radius::from(12.0),
        },
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.9, 0.95, 1.0, 0.0
        ))),
        text_color: None,
        ..Default::default()
    })
    .padding(20)
    .into()
}
