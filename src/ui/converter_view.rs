use iced::border::Radius;
use iced::widget::{Space, button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Color, Element, Length, Theme};

use crate::shared::any_unit::AnyUnit;
use crate::state::message::Message;
use crate::ui::rounded_component_styles::{
    rounded_button_style, rounded_input_style, rounded_pick_list_style,
};

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
                column![
                    row![
                        text_input("Enter value", input_value)
                            .on_input(Message::InputChanged)
                            .width(Length::Fixed(348.0))
                            .padding(8)
                            .style(rounded_input_style),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    Space::with_height(15),
                    row![
                        pick_list(units, Some(*from_unit), Message::FromUnitChanged)
                            .width(Length::Fixed(130.0))
                            .style(rounded_pick_list_style),
                        button("⇆ Swap")
                            .on_press(Message::SwapUnits)
                            .style(|theme, status| {
                                rounded_button_style(theme, status, Some(button::secondary))
                            }),
                        pick_list(units, Some(*to_unit), Message::ToUnitChanged)
                            .width(Length::Fixed(130.0))
                            .style(rounded_pick_list_style)
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                ],
                column![
                    row![
                        Space::with_width(30),
                        text(format!("{:.4}", result)).size(60),
                    ],
                    row![Space::with_width(40), text(to_unit.to_string()).size(18)]
                ],
            ],
            button(text("Convert").size(14))
                .on_press(Message::Convert)
                .style(|theme, status| {
                    rounded_button_style(theme, status, Some(button::primary))
                })
        ]
        .spacing(15)
        .align_x(Alignment::Start),
    )
    .style(|_theme: &Theme| container::Style {
        border: iced::Border {
            color: Color::from_rgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0),
            width: 1.0,
            radius: Radius::from(12.0),
        },
        background: Some(iced::Background::Color(Color::from_rgb(
            30.0 / 255.0,
            30.0 / 255.0,
            30.0 / 255.0,
        ))),
        text_color: None,
        ..Default::default()
    })
    .width(Length::Fixed(650.0))
    .padding(20)
    .into()
}
