use iced::widget::{Column, Container, button, column, text};
use iced::{Alignment, Border, Element, Length};

use crate::shared::category::Category;
use crate::shared::constant::DARK_GRAY;
use crate::state::message::Message;
use crate::ui::rounded_component_styles::rounded_button_style;

pub fn category_selector(category: Category) -> Element<'static, Message> {
    let list = Category::ALL.iter().fold(
        Column::new().spacing(4).align_x(Alignment::Start),
        |col, cat| {
            let is_selected = *cat == category;

            col.push(
                button(Container::new(text(cat.to_string()).size(12)).padding(2))
                    .on_press(Message::CategoryChanged(cat.clone()))
                    .style(move |theme, status| {
                        if is_selected {
                            rounded_button_style(theme, status, Some(button::success))
                        } else {
                            rounded_button_style(theme, status, None)
                        }
                    }),
            )
        },
    );
    column![
        Container::new(list)
            .padding(16)
            .width(Length::Fixed(230.0))
            .style(|_| iced::widget::container::Style {
                border: Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: DARK_GRAY,
                },
                background: Some(iced::Background::Color(DARK_GRAY)),
                ..Default::default()
            })
    ]
    .spacing(15)
    .align_x(Alignment::Center)
    .into()
}
